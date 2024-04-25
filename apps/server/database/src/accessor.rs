use std::sync::Arc;

use deadpool_postgres::Pool;
use futures::lock::Mutex;
use rusqlite::{params, Connection, Result};

//use postgres::{Client, Config, NoTls};

use game_types::{CharacterRaw, CharacterRawBuilder};
use tokio_postgres::Client;

use crate::trait_def::DatabaseAccess;

use crate::cornucopia::queries::reads::{get_team_user_characters_with_stats, GetTeamUserCharacters};

pub struct DatabaseAccessor {
    pub pool: Pool,
}

impl DatabaseAccess for DatabaseAccessor {
    async fn get_player_team_charas(&mut self, player_id: &str) -> Result<Vec<CharacterRaw>, ()> {
        /*
        let mut client = Config::new()
            .user("postgres")
            .password("postgres")
            .host("127.0.0.1")
            .port(5435)
            .dbname("postgres")
            .connect(NoTls)
            .unwrap();
        */

        let res = {
            let client = self.pool.get().await.unwrap(); //.lock().await.get().await.unwrap();
                                                         /*
                                                         let res = character_queries::user_team_characters()
                                                             .bind(&client, &player_id)
                                                             .all()
                                                             .await
                                                             .unwrap();
                                                         */

            let res = get_team_user_characters_with_stats()
                .bind(&client, &player_id)
                .all()
                .await
                .unwrap();

            res
        };

        //character_queries::user_team_characters().bind(&mut self.client, &"yep".to_string()).all();

        // map to CharacterRaw
        let chars = res
            .iter()
            .map(|chara| {
                CharacterRaw {
                max_hp: chara.hp as u32,
                attack: chara.attack as u32,
                defense: chara.defense as u32,
                speed: chara.speed as u32,
                experience: chara.experience as u32,
            }
    })
            .collect();

        return Ok(chars);
    }
}
