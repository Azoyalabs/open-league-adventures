use std::time::Duration;

use protodefs::pbfight::{
    client_fight_message::ClientMessage, fight_service_client::FightServiceClient, server_fight_message::Payload, ClientFightMessage, RequestNextTick, RequestStartFight
};
use tokio::time;
use tonic::Request;

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
