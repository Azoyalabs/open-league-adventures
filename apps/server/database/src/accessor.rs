use deadpool_postgres::Pool;


//use postgres::{Client, Config, NoTls};

use game_types::CharacterRaw;

use crate::cornucopia::queries::writes::{add_gold_player, update_chara_lvl_xp};
use crate::trait_def::DatabaseAccess;

use crate::cornucopia::queries::reads::{get_team_user_characters_with_stats, get_xp_required};

pub struct DatabaseAccessor {
    pub pool: Pool,
}

impl DatabaseAccess for DatabaseAccessor {
    async fn add_gold_player(&mut self, player_id: &str, gold_amount: u32) -> Result<(), ()> {
        let client = self.pool.get().await.unwrap(); //.lock().await.get().await.unwrap();
        add_gold_player()
            .bind(&client, &(gold_amount as i32), &player_id)
            .await
            .unwrap();

        return Ok(());
    }

    async fn set_chara_xp_lvl(&mut self, charaid: String, lvl: u32, xp: u32) -> Result<(), ()> {
        let client = self.pool.get().await.unwrap(); //.lock().await.get().await.unwrap();
        update_chara_lvl_xp()
            .bind(&client, &(lvl as i16), &(xp as i32), &charaid)
            .await
            .unwrap();

        return Ok(());
    }

    async fn get_xp_required(&self, lvl: u32) -> Result<u32, ()> {
        let res = {
            let client = self.pool.get().await.unwrap(); //.lock().await.get().await.unwrap();
                                                         /*
                                                         let res = character_queries::user_team_characters()
                                                             .bind(&client, &player_id)
                                                             .all()
                                                             .await
                                                             .unwrap();
                                                         */

            get_xp_required()
                .bind(&client, &(lvl as i16))
                .all()
                .await
                .unwrap()
        };

        return Ok(res[0] as u32);
    }

    async fn get_player_team_charas(&mut self, player_id: &str) -> Result<Vec<CharacterRaw>, ()> {
        let res = {
            let client = self.pool.get().await.unwrap();

            get_team_user_characters_with_stats()
                .bind(&client, &player_id)
                .all()
                .await
                .unwrap()
        };
        
        // map to CharacterRaw
        let chars = res
            .iter()
            .map(|chara| CharacterRaw {
                character_id: chara.character_id.to_owned(),
                lvl: chara.lvl as u32,
                max_hp: chara.hp as u32,
                attack: chara.attack as u32,
                defense: chara.defense as u32,
                speed: chara.speed as u32,
                experience: chara.experience as u32,
            })
            .collect();

        Ok(chars)
    }
}