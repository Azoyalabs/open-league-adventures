use std::pin::Pin;

use futures::Future;
use game_types::CharacterRaw;

pub trait DatabaseAccess {
    async fn add_gold_player(&mut self, player_id: &str, gold_amount: u32) -> Result<(), ()>;

    //async fn get_player_charas(player_id: u32) -> Result<Vec<CharacterRaw>, Box<dyn Error>>;
    async fn get_player_team_charas(&mut self, player_id: &str) -> Result<Vec<CharacterRaw>, ()>;

    async fn get_xp_required(&self, lvl: u32) -> Result<u32, ()>;

    async fn set_chara_xp_lvl(&mut self, charaid: String, lvl: u32, xp: u32) -> Result<(), ()>;
}

pub trait DbWrapper {
    fn add_gold_player(
        &mut self,
        player_id: &str,
        gold_amount: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;

    fn get_player_team_charas(
        &self,
        player_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<CharacterRaw>, ()>> + Send>>;

    fn get_xp_required(&self, lvl: u32) -> Pin<Box<dyn Future<Output = Result<u32, ()>> + Send>>;

    fn set_chara_xp_lvl(
        &mut self,
        charaid: String,
        lvl: u32,
        xp: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>>;
}

pub struct MockDbWrapper {}

impl DbWrapper for MockDbWrapper {
    fn add_gold_player(
        &mut self,
        player_id: &str,
        gold_amount: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>> {
        Box::pin(async move { Ok(()) })
    }

    fn get_player_team_charas(
        &self,
        player_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<CharacterRaw>, ()>> + Send>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn get_xp_required(&self, lvl: u32) -> Pin<Box<dyn Future<Output = Result<u32, ()>> + Send>> {
        Box::pin(async move { Ok(100) })
    }

    fn set_chara_xp_lvl(
        &mut self,
        charaid: String,
        lvl: u32,
        xp: u32,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send>> {
        Box::pin(async move { Ok(()) })
    }
}

pub async fn tryhard(client: &mut Box<dyn DbWrapper>) -> Result<(), ()> {
    return client.add_gold_player("BigFrogInc", 0).await;
}
