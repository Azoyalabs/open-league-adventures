

use game_types::CharacterRaw;


pub trait DatabaseAccess {
    //async fn get_player_charas(player_id: u32) -> Result<Vec<CharacterRaw>, Box<dyn Error>>;
    async fn get_player_team_charas(&mut self, player_id: &str) -> Result<Vec<CharacterRaw>, ()>;

    async fn get_xp_required(&self, lvl: u32) -> Result<u32, ()>; 

    async fn set_chara_xp_lvl(&mut self, charaid: String, lvl: u32, xp: u32) -> Result<(), ()>; 
}

