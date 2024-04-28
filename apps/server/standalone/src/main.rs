use std::{collections::HashMap, sync::Arc};

use database::{accessor::DatabaseAccessor, AccessorWrapper};
use deadpool_postgres::{Config, Runtime};
use dotenv::dotenv;
use fight_system::FightStatus;
use futures_util::lock::Mutex;
use game_types::Character;

use postgres::NoTls;
use rand::{rngs::SmallRng, SeedableRng};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use protodefs::pbfight::{
    client_fight_message::ClientMessage,
    fight_service_server::{FightService, FightServiceServer},
    server_fight_message::Payload,
    ClientFightMessage, EndFight, RawCharacterData, RequestFightNextTickMessage, RequestStartFight,
    ResponseFightNextTick, ServerFightMessage, StartFight,
};

use tokio_stream::StreamExt;


pub struct MyFightService {
    pub db_accessor: Arc<Mutex<AccessorWrapper>>, //Arc<Mutex<Box<dyn DatabaseAccess + Send + 'static>>>,
    pub channels: Arc<Mutex<HashMap<u32, mpsc::Sender<()>>>>,
    pub id_allocator: Arc<Mutex<u32>>,
}

#[tonic::async_trait]
impl FightService for MyFightService {
    type FightStream = ReceiverStream<Result<ServerFightMessage, Status>>;
    type RequestFightStartStream = ReceiverStream<Result<ServerFightMessage, Status>>;

    async fn fight(
        &self,
        request: Request<tonic::Streaming<ClientFightMessage>>,
    ) -> Result<Response<Self::FightStream>, Status> {
        let mut client_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(4);

        let db_accessor = { self.db_accessor.clone() };

        tokio::spawn(async move {
            let mut rng = SmallRng::from_entropy();
            let mut fight_ongoing = true;

            let mut player_charas_ok: Vec<Character> = vec![];

            let mut enemy_chars: Vec<Character> = vec![Character {
                max_hp: 6,
                current_hp: 6,
                attack: 6,
                defense: 6,
                speed: 6,
            }];
            let mut player_speed_acc = vec![0; player_charas_ok.len()];
            let mut enemy_speed_acc = vec![0; enemy_chars.len()];

            while let Some(msg) = client_stream.next().await {
                if let Ok(client_msg) = msg {
                    match client_msg.client_message.unwrap() {
                        ClientMessage::RequestNextTick(_) => {
                            if !fight_ongoing {
                                continue;
                            }

                            let (tick_data, fight_status) = fight_system::tick(
                                &mut player_charas_ok,
                                &mut enemy_chars,
                                &mut player_speed_acc,
                                &mut enemy_speed_acc,
                                &mut rng,
                            );

                            // process next tick of the game
                            tx.send(Ok(ServerFightMessage {
                                payload: Some(Payload::FightAction(tick_data.to_protobuf())),
                            }))
                            .await
                            .unwrap();

                            // check fight status
                            if !matches!(fight_status, FightStatus::Ongoing) {
                                // signal that fight has ended
                                fight_ongoing = false;
                                /*let winner = */
                                match fight_status {
                                    FightStatus::Ongoing {} => panic!("never"),
                                    FightStatus::FightEnded { player_won } => {
                                        tx.send(Ok(ServerFightMessage {
                                            payload: Some(Payload::EndFight(EndFight {
                                                is_player_victory: player_won,
                                                experience: 8,
                                            })),
                                        }))
                                        .await
                                        .unwrap();
                                    }
                                };
                            }
                        }
                        ClientMessage::RequestStartFight(rq) => {
                            let player_charas_from_server = {
                                let mut acc = db_accessor.lock().await; //.unwrap();
                                acc.get_player_team_charas(&rq.player_id).await.unwrap()
                            };

                            player_charas_ok = player_charas_from_server
                                .iter()
                                .map(|chara| chara.to_character())
                                .collect();

                            player_speed_acc = vec![0; player_charas_ok.len()];

                            let player_charas: Vec<RawCharacterData> = player_charas_from_server
                                .iter()
                                .map(|chara| RawCharacterData {
                                    max_hp: chara.max_hp,
                                    attack: chara.attack,
                                    defense: 0,
                                    speed: chara.speed,
                                })
                                .collect();
                            tx.send(Ok(ServerFightMessage {
                                payload: Some(Payload::StartFight(StartFight {
                                    fight_id: 0,
                                    player_characters: player_charas.clone(),
                                    enemy_characters: vec![],
                                })),
                            }))
                            .await
                            .unwrap();
                        }
                    }
                } else {
                    // stream closed, break out
                    println!("Client has dropped, closing stream");
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn request_fight_start(
        &self,
        request: Request<RequestStartFight>,
    ) -> Result<Response<Self::RequestFightStartStream>, Status> {
        println!("Received request to start new fight");
        let (tx, rx) = mpsc::channel(4);

        let (tx_next_tick, mut rx_next_tick) = mpsc::channel(4);

        // get the id for this fight and increment
        let fight_id = {
            let mut lock = self.id_allocator.lock().await;
            let fight_id = *lock;
            *lock += 1;
            println!("Allocated fight id: {}", fight_id);
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

            let player_charas_from_server = {
                let mut acc = db_accessor.lock().await; //.unwrap();
                acc.get_player_team_charas(&player_id).await.unwrap()
            };

            println!("Nb Charas Player: {}", player_charas_from_server.len());

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
            tx.send(Ok(ServerFightMessage {
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
                tx.send(Ok(ServerFightMessage {
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
                            println!("Fight has ended, notifying player");
                            tx.send(Ok(ServerFightMessage {
                                payload: Some(Payload::EndFight(EndFight {
                                    is_player_victory: player_won,
                                    experience: 8,
                                })),
                            }))
                            .await
                            .unwrap();
                            break;
                        }
                    };
                }
            }
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn request_fight_next_tick(
        &self,
        request: Request<RequestFightNextTickMessage>,
    ) -> Result<Response<ResponseFightNextTick>, Status> {
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

        return Ok(Response::new(ResponseFightNextTick {}));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.user = Some(std::env::var("SUPABASE_DB_USER").unwrap()); //.expect("SUPABASE_DB_USER must be set."));
    cfg.password =
        Some(std::env::var("SUPABASE_DB_PASSWORD").unwrap()); //.expect("SUPABASE_DB_PASSWORD must be set."));
    cfg.host = Some(std::env::var("SUPABASE_DB_HOST").unwrap()); //.expect("SUPABASE_DB_HOST must be set."));
    cfg.port = Some(
        std::env::var("SUPABASE_DB_PORT")
            .expect("SUPABASE_DB_PORT must be set.")
            .parse()
            .unwrap(),
    );
    cfg.dbname = Some(std::env::var("SUPABASE_DB_NAME").expect("SUPABASE_DB_NAME must be set."));

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let addr = "[::1]:10000".parse().unwrap();
    let db_accessor = DatabaseAccessor {
        pool, //Arc::new(Mutex::new(pool))
    };

    let wrapped = AccessorWrapper::Live(db_accessor);

    //let addr = "[::1]:50051".parse()?;
    let fight_service = MyFightService {
        db_accessor: Arc::new(Mutex::new(wrapped)),
        channels: Arc::new(Mutex::new(HashMap::new())),
        id_allocator: Arc::new(Mutex::new(0)),
    };


    let svc = FightServiceServer::new(fight_service);
    Server::builder()
    .accept_http1(true)
    //.layer(GrpcWebLayer::new())
    //.add_service(svc)
    .add_service(tonic_web::enable(svc))
    .serve(addr).await?;

    return Ok(());
}
