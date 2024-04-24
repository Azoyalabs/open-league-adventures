use std::fs::File;
use std::io::Read;
use std::time::Instant;

use deadpool_postgres::{Config, Runtime};
use dotenv::dotenv;
use game_types::Character;

use database::cornucopia::queries::{reads::get_team_user_characters_with_stats, writes::create_character};

use database::cornucopia::queries::migrations::set_experience;

use postgres::{Client, NoTls};

use tokio::{runtime::Handle, sync::mpsc};
use tonic::{transport::Server, Request, Response, Status};


use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ExperienceData {
    pub level: i16,
    pub xp: i32
}


// use this to feed db
#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.user = Some(std::env::var("SUPABASE_DB_USER").expect("SUPABASE_DB_USER must be set."));
    cfg.password = Some(std::env::var("SUPABASE_DB_PASSWORD").expect("SUPABASE_DB_USER must be set."));
    cfg.host =  Some(std::env::var("SUPABASE_DB_HOST").expect("SUPABASE_DB_HOST must be set."));
    cfg.port = Some(std::env::var("SUPABASE_DB_PORT").expect("SUPABASE_DB_PORT must be set.").parse().unwrap());
    cfg.dbname = Some(std::env::var("SUPABASE_DB_NAME").expect("SUPABASE_DB_NAME must be set."));

    
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut client = pool.get().await.unwrap();

    /*
    // read experience file 
    let experiences: Vec<ExperienceData> = {
        let mut file = File::open("data/experience_table.json").unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
     
        serde_json::from_str(&buff).unwrap()
    };


    let curr_time = Instant::now();
    let mut tx = client.transaction().await.unwrap();

    let elapsed_tx_creation = curr_time.elapsed();

    //let stmt = set_experience();
    for exp in experiences {
        if exp.level != 1 {
            set_experience().bind(&tx, &(exp.level - 1), &exp.xp).await.unwrap();
        }
    }

    let elapsed_binds = curr_time.elapsed();

    tx.commit().await.unwrap();
    
    let elapsed_end = curr_time.elapsed();

    println!("Time for create tx: {}", elapsed_tx_creation.as_secs_f32());
    println!("Time for set_experiences: {}", elapsed_binds.as_secs_f32());
    println!("Total time: {}", elapsed_end.as_secs_f32());
    */

    
    let res = get_team_user_characters_with_stats()
        .bind(&mut client, &"BigFrogInc".to_string())
        .all()
        .await
        .unwrap();


    println!("chars: {:?}", res);
    
}
