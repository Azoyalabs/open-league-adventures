use std::time::Duration;

use protodefs::pbfight::{
    fight_service_client::FightServiceClient,
    server_fight_message::Payload, RequestFightNextTickMessage, RequestStartFight,
};
use tokio::sync::mpsc;
use tonic::Request;


enum MyMessage {
    StartFight(u32),
    NextTick,
    EndFight,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let start = time::Instant::now();
    let (tx, mut rx) = mpsc::channel(4);

    //let server_url = "http://[::1]:10000";
    let server_url = "http://207.154.205.115:10000";

    let handle1 = tokio::spawn(async move {
        let mut client = FightServiceClient::connect(server_url)
            .await
            .unwrap();


        let mut fight_id = None;
        
        while let Some(msg) = rx.recv().await {
            /*
            if msg {
                break;
            }
            */

            match msg {
                MyMessage::StartFight(id) => {
                    println!("fight id is: {}", id);
                    fight_id = Some(id)
                }
                MyMessage::EndFight => break,
                MyMessage::NextTick => (),
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            client
                .request_fight_next_tick(Request::new(RequestFightNextTickMessage {
                    fight_id: fight_id.unwrap(),
                }))
                .await
                .unwrap();
        }
    });

    let handle_2 = tokio::spawn(async move {
        let mut client = FightServiceClient::connect(server_url)
            .await
            .unwrap();

        let response = client
            .request_fight_start(Request::new(RequestStartFight {
                player_id: "BigFrogInc".into(),
            }))
            .await
            .unwrap();

        let mut inbound = response.into_inner();

        while let Some(msg) = inbound.message().await.unwrap() {
            println!("RESPONSE = {:?}\n", msg);

            match msg.payload.unwrap() {
                Payload::EndFight(_) => {
                    tx.send(MyMessage::EndFight).await.unwrap();
                    break;
                }
                Payload::StartFight(data) => {
                    tx.send(MyMessage::StartFight(data.fight_id)).await.unwrap();
                }
                Payload::FightAction(_) => tx.send(MyMessage::NextTick).await.unwrap(),
            }

        }
    });

    tokio::select!(
        _ = handle1 => {},
        _ = handle_2 => {}
    );

    Ok(())
}
