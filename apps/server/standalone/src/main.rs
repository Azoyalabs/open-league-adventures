use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use database::{accessor::DatabaseAccessor, live_db_wrapper::LiveDbWrapper, trait_def::{DatabaseAccess, DbWrapper}, AccessorWrapper};
use deadpool_postgres::{Config, Runtime};
use dotenv::dotenv;
use fight_system::FightStatus;
use futures_util::lock::Mutex;
use game_types::Character;

use postgres::NoTls;
use rand::{rngs::SmallRng, SeedableRng};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status,
};

use protodefs::pbfight::{
    fight_service_server::{FightService, FightServiceServer},
    start_fight_response::Payload,
    EndFight, NextTickRequest, NextTickResponse, RawCharacterData, StartFight, StartFightRequest,
    StartFightResponse,
};

pub struct MyFightService {
    pub db_accessor: Arc<Mutex<Box<dyn DbWrapper + Send>>>, //Arc<Mutex<Box<dyn DatabaseAccess + Send + 'static>>>,
    pub channels: Arc<Mutex<HashMap<u32, mpsc::Sender<()>>>>,
    pub id_allocator: Arc<Mutex<u32>>,
}

#[tonic::async_trait]
impl FightService for MyFightService {
    type StartFightStream = ReceiverStream<Result<StartFightResponse, Status>>;

    async fn start_fight(
        &self,
        request: Request<StartFightRequest>,
    ) -> Result<Response<Self::StartFightStream>, Status> {
        println!("Received request to start new fight");
        let (tx, rx) = mpsc::channel(4);

        let (tx_next_tick, mut rx_next_tick) = mpsc::channel(4);

        // get the id for this fight and increment
        let fight_id = {
            let mut lock = self.id_allocator.lock().await;
            let fight_id = *lock;
            *lock += 1;
            //println!("Allocated fight id: {}", fight_id);
            fight_id
        };

        // record the Sender for communication between processes
        {
            self.channels.lock().await.insert(fight_id, tx_next_tick);
        }

        let db_accessor = { self.db_accessor.clone() };

        let player_id = request.into_inner().player_id;

        tokio::spawn(async move {
            let mut rng = SmallRng::from_entropy();

            let mut enemy_chars: Vec<Character> = vec![Character {
                max_hp: 6,
                current_hp: 6,
                attack: 6,
                defense: 6,
                speed: 6,
            }];

            let player_charas_from_server: Vec<game_types::CharacterRaw> = {
                let acc = db_accessor.lock().await; //.unwrap();
                acc.get_player_team_charas(&player_id).await.unwrap()
                //vec![]
            };

            let mut player_charas_ok: Vec<Character> = player_charas_from_server
                .iter()
                .map(|chara| chara.to_character())
                .collect();

            let mut player_speed_acc = vec![0; player_charas_ok.len()];
            let mut enemy_speed_acc = vec![0; enemy_chars.len()];

            let player_charas: Vec<RawCharacterData> = player_charas_from_server
                .iter()
                .map(|chara| RawCharacterData {
                    max_hp: chara.max_hp,
                    attack: chara.attack,
                    defense: chara.defense,
                    speed: chara.speed,
                })
                .collect();

            let enemy_charas_to_send: Vec<RawCharacterData> = enemy_chars
                .iter()
                .map(|chara| RawCharacterData {
                    max_hp: chara.max_hp,
                    attack: chara.attack,
                    defense: chara.defense,
                    speed: chara.speed,
                })
                .collect();

            // send to player that fight is ready to proceed
            tx.send(Ok(StartFightResponse {
                payload: Some(Payload::StartFight(StartFight {
                    fight_id,
                    player_characters: player_charas.clone(),
                    enemy_characters: enemy_charas_to_send,
                })),
            }))
            .await
            .unwrap();

            while rx_next_tick.recv().await.is_some() {
                //println!("received request for new tick");
                let (tick_data, fight_status) = fight_system::tick(
                    &mut player_charas_ok,
                    &mut enemy_chars,
                    &mut player_speed_acc,
                    &mut enemy_speed_acc,
                    &mut rng,
                );

                // process next tick of the game
                tx.send(Ok(StartFightResponse {
                    payload: Some(Payload::FightAction(tick_data.to_protobuf())),
                }))
                .await
                .unwrap();

                // check fight status
                if !matches!(fight_status, FightStatus::Ongoing) {
                    // signal that fight has ended
                    //fight_ongoing = false;
                    /*let winner = */
                    match fight_status {
                        FightStatus::Ongoing {} => panic!("never"),
                        FightStatus::FightEnded { player_won } => {
                            let xp_gained = 8;
                            let gold_gained = 10;

                            println!("Fight has ended, notifying player");
                            tx.send(Ok(StartFightResponse {
                                payload: Some(Payload::EndFight(EndFight {
                                    is_player_victory: player_won,
                                    experience: xp_gained,
                                    gold: gold_gained,
                                })),
                            }))
                            .await
                            .unwrap();

                            // update xp on database
                            let mut all_levels: HashSet<u32> = HashSet::new();
                            for c in &player_charas_from_server {
                                all_levels.insert(c.lvl.clone());
                            }

                            let mut lvl_to_req: HashMap<u32, u32> = HashMap::new();
                            {
                                {
                                    let db_access = db_accessor.lock().await;
                                    for lvl in all_levels {
                                        lvl_to_req.insert(
                                            lvl,
                                            db_access.get_xp_required(lvl).await.unwrap(),
                                        );
                                        
                                    }
                                }
                            }

                            // compute level ups, and update on db
                            for c in &player_charas_from_server {
                                let mut new_xp = c.experience + xp_gained;
                                let mut new_lvl: u32 = c.lvl;
                                let req_xp = *lvl_to_req.get(&new_lvl).unwrap();
                                if new_xp >= req_xp {
                                    new_xp -= req_xp;
                                    new_lvl += 1;
                                }

                                // call to db
                                {
                                    
                                    let mut db_access = db_accessor.lock().await;
                                    db_access
                                        .set_chara_xp_lvl(c.character_id.clone(), new_lvl, new_xp)
                                        .await
                                        .unwrap();
                                    
                                }
                            }

                            // update gold on database
                            {
                                
                                let mut db_access = db_accessor.lock().await;
                                db_access
                                    .add_gold_player(&player_id, gold_gained)
                                    .await
                                    .unwrap();
                                
                            }

                            break;
                        }
                    };
                }
            }
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn next_tick(
        &self,
        request: Request<NextTickRequest>,
    ) -> Result<Response<NextTickResponse>, Status> {
        let tx = {
            self.channels
                .lock()
                .await
                .get(&request.into_inner().fight_id)
                .unwrap()
                .clone()
            /*
            .send(())
            .await
            .unwrap();
            */
        };

        tx.send(()).await.unwrap();

        return Ok(Response::new(NextTickResponse {}));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.user = Some(std::env::var("SUPABASE_DB_USER").expect("SUPABASE_DB_USER must be set."));
    cfg.password =
        Some(std::env::var("SUPABASE_DB_PASSWORD").expect("SUPABASE_DB_PASSWORD must be set."));
    cfg.host = Some(std::env::var("SUPABASE_DB_HOST").expect("SUPABASE_DB_HOST must be set."));
    cfg.port = Some(
        std::env::var("SUPABASE_DB_PORT")
            .expect("SUPABASE_DB_PORT must be set.")
            .parse()
            .unwrap(),
    );
    cfg.dbname = Some(std::env::var("SUPABASE_DB_NAME").expect("SUPABASE_DB_NAME must be set."));


    let addr = "0.0.0.0:10000".parse().unwrap(); //"[::1]:10000".parse().unwrap();
    //let addr = "[::1]:10000".parse().unwrap();


    //let wrapped = AccessorWrapper::Live(db_accessor);
    let wrapped = LiveDbWrapper {
        pool: cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
    };

    //let addr = "[::1]:50051".parse()?;
    let fight_service = MyFightService {
        db_accessor: Arc::new(Mutex::new(Box::new(wrapped))),
        channels: Arc::new(Mutex::new(HashMap::new())),
        id_allocator: Arc::new(Mutex::new(0)),
    };

    // set up tls
    /*
    let cert_path =
        std::env::var("TLS_CERTIFICATE_PATH").expect("TLS_CERTIFICATE_PATH must be set.");
    let key_path = std::env::var("TLS_CERTIFICATE_KEY").expect("TLS_CERTIFICATE_KEY must be set.");
    let cert = tokio::fs::read(cert_path).await?;
    let key = tokio::fs::read(key_path).await?;

    let tls = ServerTlsConfig::new().identity(Identity::from_pem(cert, key));
    */

    let svc = FightServiceServer::new(fight_service);
    Server::builder()
        /*
        .tls_config(tls)
        .unwrap()
        */
        .accept_http1(true)
        //.layer(GrpcWebLayer::new())
        //.add_service(svc)
        .add_service(tonic_web::enable(svc))
        .serve(addr)
        .await?;

    return Ok(());
}
