use sqlx::{Database, FromRow, PgPool, Pool, Postgres};




#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub id: i64,
}


pub struct SqlxWrapper {
    pub pool: PgPool
}

impl SqlxWrapper {
    pub async fn get_team_user_characters(&self, player_id: &str) -> User {
        let query = "SELECT Character.*
        FROM PlayerTeam
        JOIN Character ON PlayerTeam.playerId = Character.playerID
        WHERE PlayerTeam.playerId = $1";

        let res = sqlx::query(
            query
        ).bind(player_id).fetch_one(&self.pool).await.unwrap();

        return User::from_row(&res).unwrap();
    }
}