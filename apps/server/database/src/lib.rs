#![allow(async_fn_in_trait)]
#![allow(non_snake_case)]

use accessor::DatabaseAccessor;
use game_types::CharacterRaw;
use mock_accessor::MockDatabaseAccessor;
use trait_def::DatabaseAccess;



pub mod accessor;
pub mod mock_accessor;
pub mod trait_def;

pub mod cornucopia;


pub enum AccessorWrapper {
    Mock(MockDatabaseAccessor),
    Live(DatabaseAccessor)
}

impl AccessorWrapper {
    pub async fn get_player_team_charas(
        &mut self,
        player_id: &str,
    ) -> Result<Vec<CharacterRaw>, ()> {
        match self {
            AccessorWrapper::Live(live) => {
                live.get_player_team_charas(player_id).await
            },
            AccessorWrapper::Mock(mock) => {
                mock.get_player_team_charas(player_id).await
            }
        }
    }
}