

use game_types::{CharacterRaw, CharacterRawBuilder};

use crate::trait_def::DatabaseAccess;

pub struct MockDatabaseAccessor {}

impl DatabaseAccess for MockDatabaseAccessor {
    async fn get_player_team_charas(
        &mut self,
        _player_id: &str,
    ) -> Result<Vec<CharacterRaw>, ()> {
        let mut team = vec![];
        team.push(
            CharacterRawBuilder::new()
                .hp(16)
                .speed(8)
                .attack(8)
                .build(),
        );
        team.push(
            CharacterRawBuilder::new()
                .hp(12)
                .speed(10)
                .attack(6)
                .build(),
        );
        team.push(
            CharacterRawBuilder::new()
                .hp(20)
                .speed(4)
                .attack(12)
                .build(),
        );

        Ok(team)
    }
}
