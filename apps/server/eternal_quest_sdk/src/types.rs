#[derive(sqlx::FromRow)]
pub struct Player {
    pub id: String,
    pub clanid: Option<i32>,
    pub gold: i32
}


#[derive(sqlx::FromRow)]
pub struct XpRequirement {
    pub lvl: i16,
    pub required: i32  
}


#[derive(sqlx::FromRow)]
pub struct Character {
    pub id: String,
    pub playerid: String,
    pub archetypeid: String,
    pub lvl: i16,
    pub experience: i32
}

#[derive(sqlx::FromRow)]
pub struct Stats {
    pub attack: i16,
    pub defense: i16,
    pub speed: i16,
    pub hp: i16
}


#[derive(sqlx::FromRow)]
pub struct Pawn {
    pub id: String,
    pub lvl: i16,
    pub experience: i32,
    pub playerid: String,
    pub archetypeid: String,
    pub attack: i16,
    pub defense: i16,
    pub speed: i16,
    pub hp: i16,
    pub class: String,
    pub tonextlevel: i32,
    //pub isinteam: bool
}


#[derive(sqlx::FromRow)]
pub struct Referral {
    pub referrer_id: String,
    pub referred_id: String
}

