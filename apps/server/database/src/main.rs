use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use database::cornucopia::queries::writes::create_archetype_stats;
use deadpool_postgres::{Config, Runtime};
use dotenv::dotenv;

use postgres::NoTls;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ExperienceData {
    pub level: i16,
    pub xp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct StatsPerLevel {
    pub level: i16,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub attack: i16,
    pub defense: i16,
    pub speed: i16,
    pub hp: i16,
}

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Acquire,
};
use tokio::time::Instant;

// use this to feed db
#[tokio::main]
async fn main() {
    /*
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.user = Some(std::env::var("SUPABASE_DB_USER").expect("SUPABASE_DB_USER must be set."));
    cfg.password =
        Some(std::env::var("SUPABASE_DB_PASSWORD").expect("SUPABASE_DB_USER must be set."));
    cfg.host = Some(std::env::var("SUPABASE_DB_HOST").expect("SUPABASE_DB_HOST must be set."));
    cfg.port = Some(
        std::env::var("SUPABASE_DB_PORT")
            .expect("SUPABASE_DB_PORT must be set.")
            .parse()
            .unwrap(),
    );
    cfg.dbname = Some(std::env::var("SUPABASE_DB_NAME").expect("SUPABASE_DB_NAME must be set."));

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let mut client = pool.get().await.unwrap();
    */

    let opt = PgConnectOptions::new()
    .host("postgres://postgres.bgizgttxkuhgeryuhdsg:EternalQuestStrongPassword@aws-0-eu-central-1.pooler.supabase.com:5432/postgres")
    .port(5432)
    .database("postgres")
    .password("EternalQuestStrongPassword")
    .username("postgres.ytxbulmxagfnaddyrclj");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres.bgizgttxkuhgeryuhdsg:EternalQuestStrongPassword@aws-0-eu-central-1.pooler.supabase.com:5432/postgres")
        //.connect_with(opt)
        .await
        .unwrap();

    // read experience file
    let experiences: Vec<ExperienceData> = {
        let mut file = File::open("data/experience_table.json").unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        serde_json::from_str(&buff).unwrap()
    };

    let lvls: Vec<i16> = experiences.iter().map(|e| e.level).collect();
    let xps: Vec<i32> = experiences.iter().map(|e| e.xp).collect();

    let curr_time = Instant::now();
    //let mut conn = pool.acquire().await.unwrap();
    //let mut tx = pool.begin().await.unwrap();

    let elapsed_tx_creation = curr_time.elapsed();

    /*
    //let stmt = set_experience();
    for exp in experiences {
        if exp.level != 1 {
            sqlx::query(
                "INSERT INTO
                ExperienceRequirement(lvl, requiredNextLevel)
            VALUES
                ($1, $2);",
            )
            .bind(exp.level - 1)
            .bind(exp.xp)
            .execute(&pool)
            .await
            .unwrap();
        }
    }
    */

    let sql = r"
    INSERT INTO
    ExperienceRequirement(lvl, requiredNextLevel)
    SELECT * FROM UNNEST
    ($1::INT2[], $2::INT4[]);";

    let yap = sqlx::query(sql)
        .bind(lvls)
        .bind(xps)
        .execute(&pool)
        .await
        .unwrap();

    let elapsed_binds = curr_time.elapsed();

    //tx.commit().await.unwrap();

    let elapsed_end = curr_time.elapsed();

    println!("Time for create tx: {}", elapsed_tx_creation.as_secs_f32());
    println!("Time for set_experiences: {}", elapsed_binds.as_secs_f32());
    println!("Total time: {}", elapsed_end.as_secs_f32());
}

/*
// use this to feed db
#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.user = Some(std::env::var("SUPABASE_DB_USER").expect("SUPABASE_DB_USER must be set."));
    cfg.password =
        Some(std::env::var("SUPABASE_DB_PASSWORD").expect("SUPABASE_DB_USER must be set."));
    cfg.host = Some(std::env::var("SUPABASE_DB_HOST").expect("SUPABASE_DB_HOST must be set."));
    cfg.port = Some(
        std::env::var("SUPABASE_DB_PORT")
            .expect("SUPABASE_DB_PORT must be set.")
            .parse()
            .unwrap(),
    );
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
            set_experience()
                .bind(&tx, &(exp.level - 1), &exp.xp)
                .await
                .unwrap();
        }
    }

    let elapsed_binds = curr_time.elapsed();

    tx.commit().await.unwrap();

    let elapsed_end = curr_time.elapsed();

    println!("Time for create tx: {}", elapsed_tx_creation.as_secs_f32());
    println!("Time for set_experiences: {}", elapsed_binds.as_secs_f32());
    println!("Total time: {}", elapsed_end.as_secs_f32());
    */

    // read stats file
    let stats_container: HashMap<String, Vec<StatsPerLevel>> = {
        let mut file = File::open("data/processed_stats.json").unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        serde_json::from_str(&buff).unwrap()
    };

    let tx = client.transaction().await.unwrap();

    //let stmt = set_experience();
    for (class_id, class_data) in stats_container {
        for per_level_data in class_data {
            create_archetype_stats()
                .bind(
                    &tx,
                    &per_level_data.level,
                    &class_id,
                    &per_level_data.stats.attack,
                    &per_level_data.stats.defense,
                    &per_level_data.stats.speed,
                    &per_level_data.stats.hp,
                )
                .await
                .unwrap();
        }
    }

    tx.commit().await.unwrap();

    /*
    let res = get_team_user_characters_with_stats()
        .bind(&mut client, &"BigFrogInc".to_string())
        .all()
        .await
        .unwrap();


    println!("chars: {:?}", res);
    */
}
*/
