use std::time::Duration;

use protodefs::pbfight::{
    client_fight_message::ClientMessage, fight_service_client::FightServiceClient,
    server_fight_message::Payload, ClientFightMessage, RequestFightNextTickMessage,
    RequestNextTick, RequestStartFight,
};
use tokio::{sync::mpsc, time};
use tonic::Request;

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FightServiceClient::connect("http://[::1]:10000").await?;

    //let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        yield ClientFightMessage {
            client_message: Some(
                ClientMessage::RequestStartFight(RequestStartFight{
                    player_id: "BigFrogInc".to_string()
                })
            )
        };
        interval.tick().await;


        while let time = interval.tick().await {
            yield ClientFightMessage {
                client_message: Some(
                    ClientMessage::RequestNextTick(RequestNextTick{})
                )
            }
        }
    };

    let response = client.fight(Request::new(outbound)).await.unwrap();

    let mut inbound = response.into_inner();
    while let Some(msg) = inbound.message().await? {
        println!("RESPONSE = {:?}\n", msg);

        if matches!(msg.payload.unwrap(), Payload::EndFight(_)) {
            println!("fite has ended, closing channel from client");
            break;
        }
    }

    println!("WE ESCAPED THE LOOOOOOOPPPPP");

    Ok(())
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let start = time::Instant::now();
    let (mut tx, mut rx) = mpsc::channel(4);

    let handle1 = tokio::spawn(async move {
        let mut client = FightServiceClient::connect("http://[::1]:10000")
            .await
            .unwrap();

        while let Some(msg) = rx.recv().await {
            if msg {
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            client
                .request_fight_next_tick(Request::new(RequestFightNextTickMessage { fight_id: 0 }))
                .await
                .unwrap();
        }
    });

    let handle_2 = tokio::spawn(async move {
        let mut client = FightServiceClient::connect("http://[::1]:10000")
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

            if matches!(msg.payload.unwrap(), Payload::EndFight(_)) {
                tx.send(true).await.unwrap();
                break;
            } else {
                tx.send(false).await.unwrap();
            }
        }
    });

    tokio::select!(
        _ = handle1 => {},
        _ = handle_2 => {}
    );

    Ok(())
}
