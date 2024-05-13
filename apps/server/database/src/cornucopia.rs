// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod migrations
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct SetExperienceParams<> { pub lvl: i16,pub experience: i32,}pub fn set_experience() -> SetExperienceStmt
{ SetExperienceStmt(cornucopia_async::private::Stmt::new("INSERT INTO
    ExperienceRequirement(lvl, requiredNextLevel)
VALUES
    ($1, $2)")) } pub struct
SetExperienceStmt(cornucopia_async::private::Stmt); impl SetExperienceStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
lvl: &'a i16,experience: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[lvl,experience,]).await
} }impl <'a, C: GenericClient + Send + Sync, >
cornucopia_async::Params<'a, SetExperienceParams<>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for SetExperienceStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    SetExperienceParams<>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.lvl,&params.experience,)) }
}}pub mod reads
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct GetAllUserCharacters
{ pub id : String,pub playerid : String,pub archetypeid : String,pub experience : i32,pub lvl : i16,}pub struct GetAllUserCharactersBorrowed<'a> { pub id : &'a str,pub playerid : &'a str,pub archetypeid : &'a str,pub experience : i32,pub lvl : i16,}
impl<'a> From<GetAllUserCharactersBorrowed<'a>> for GetAllUserCharacters
{
    fn from(GetAllUserCharactersBorrowed { id,playerid,archetypeid,experience,lvl,}: GetAllUserCharactersBorrowed<'a>) ->
    Self { Self { id: id.into(),playerid: playerid.into(),archetypeid: archetypeid.into(),experience,lvl,} }
}pub struct GetAllUserCharactersQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetAllUserCharactersBorrowed,
    mapper: fn(GetAllUserCharactersBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetAllUserCharactersQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetAllUserCharactersBorrowed) -> R) ->
    GetAllUserCharactersQuery<'a,C,R,N>
    {
        GetAllUserCharactersQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetAllUserCharactersWithStats
{ pub id : String,pub experience : i32,pub lvl : i16,pub archetypeid : String,pub attack : i16,pub defense : i16,pub speed : i16,pub hp : i16,}pub struct GetAllUserCharactersWithStatsBorrowed<'a> { pub id : &'a str,pub experience : i32,pub lvl : i16,pub archetypeid : &'a str,pub attack : i16,pub defense : i16,pub speed : i16,pub hp : i16,}
impl<'a> From<GetAllUserCharactersWithStatsBorrowed<'a>> for GetAllUserCharactersWithStats
{
    fn from(GetAllUserCharactersWithStatsBorrowed { id,experience,lvl,archetypeid,attack,defense,speed,hp,}: GetAllUserCharactersWithStatsBorrowed<'a>) ->
    Self { Self { id: id.into(),experience,lvl,archetypeid: archetypeid.into(),attack,defense,speed,hp,} }
}pub struct GetAllUserCharactersWithStatsQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetAllUserCharactersWithStatsBorrowed,
    mapper: fn(GetAllUserCharactersWithStatsBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetAllUserCharactersWithStatsQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetAllUserCharactersWithStatsBorrowed) -> R) ->
    GetAllUserCharactersWithStatsQuery<'a,C,R,N>
    {
        GetAllUserCharactersWithStatsQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetTeamUserCharacters
{ pub id : String,pub playerid : String,pub archetypeid : String,pub experience : i32,pub lvl : i16,}pub struct GetTeamUserCharactersBorrowed<'a> { pub id : &'a str,pub playerid : &'a str,pub archetypeid : &'a str,pub experience : i32,pub lvl : i16,}
impl<'a> From<GetTeamUserCharactersBorrowed<'a>> for GetTeamUserCharacters
{
    fn from(GetTeamUserCharactersBorrowed { id,playerid,archetypeid,experience,lvl,}: GetTeamUserCharactersBorrowed<'a>) ->
    Self { Self { id: id.into(),playerid: playerid.into(),archetypeid: archetypeid.into(),experience,lvl,} }
}pub struct GetTeamUserCharactersQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetTeamUserCharactersBorrowed,
    mapper: fn(GetTeamUserCharactersBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetTeamUserCharactersQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetTeamUserCharactersBorrowed) -> R) ->
    GetTeamUserCharactersQuery<'a,C,R,N>
    {
        GetTeamUserCharactersQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetTeamUserCharactersWithStats
{ pub character_id : String,pub experience : i32,pub lvl : i16,pub archetypeid : String,pub attack : i16,pub defense : i16,pub speed : i16,pub hp : i16,}pub struct GetTeamUserCharactersWithStatsBorrowed<'a> { pub character_id : &'a str,pub experience : i32,pub lvl : i16,pub archetypeid : &'a str,pub attack : i16,pub defense : i16,pub speed : i16,pub hp : i16,}
impl<'a> From<GetTeamUserCharactersWithStatsBorrowed<'a>> for GetTeamUserCharactersWithStats
{
    fn from(GetTeamUserCharactersWithStatsBorrowed { character_id,experience,lvl,archetypeid,attack,defense,speed,hp,}: GetTeamUserCharactersWithStatsBorrowed<'a>) ->
    Self { Self { character_id: character_id.into(),experience,lvl,archetypeid: archetypeid.into(),attack,defense,speed,hp,} }
}pub struct GetTeamUserCharactersWithStatsQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetTeamUserCharactersWithStatsBorrowed,
    mapper: fn(GetTeamUserCharactersWithStatsBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetTeamUserCharactersWithStatsQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetTeamUserCharactersWithStatsBorrowed) -> R) ->
    GetTeamUserCharactersWithStatsQuery<'a,C,R,N>
    {
        GetTeamUserCharactersWithStatsQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn get_all_user_characters() -> GetAllUserCharactersStmt
{ GetAllUserCharactersStmt(cornucopia_async::private::Stmt::new("SELECT *
from Character
WHERE Character.playerId = $1")) } pub struct
GetAllUserCharactersStmt(cornucopia_async::private::Stmt); impl GetAllUserCharactersStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
playerid: &'a T1,) -> GetAllUserCharactersQuery<'a,C,
GetAllUserCharacters, 1>
{
    GetAllUserCharactersQuery
    {
        client, params: [playerid,], stmt: &mut self.0, extractor:
        |row| { GetAllUserCharactersBorrowed { id: row.get(0),playerid: row.get(1),archetypeid: row.get(2),experience: row.get(3),lvl: row.get(4),} }, mapper: |it| { <GetAllUserCharacters>::from(it) },
    }
} }pub fn get_all_user_characters_with_stats() -> GetAllUserCharactersWithStatsStmt
{ GetAllUserCharactersWithStatsStmt(cornucopia_async::private::Stmt::new("SELECT Character.id, Character.experience, ArchetypeStats.*
FROM Character
JOIN ArchetypeStats ON Character.archetypeID = ArchetypeStats.archetypeID
WHERE Character.playerId = $1")) } pub struct
GetAllUserCharactersWithStatsStmt(cornucopia_async::private::Stmt); impl GetAllUserCharactersWithStatsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
playerId: &'a T1,) -> GetAllUserCharactersWithStatsQuery<'a,C,
GetAllUserCharactersWithStats, 1>
{
    GetAllUserCharactersWithStatsQuery
    {
        client, params: [playerId,], stmt: &mut self.0, extractor:
        |row| { GetAllUserCharactersWithStatsBorrowed { id: row.get(0),experience: row.get(1),lvl: row.get(2),archetypeid: row.get(3),attack: row.get(4),defense: row.get(5),speed: row.get(6),hp: row.get(7),} }, mapper: |it| { <GetAllUserCharactersWithStats>::from(it) },
    }
} }pub fn get_team_user_characters() -> GetTeamUserCharactersStmt
{ GetTeamUserCharactersStmt(cornucopia_async::private::Stmt::new("SELECT Character.*
FROM PlayerTeam
JOIN Character ON PlayerTeam.playerId = Character.playerID
WHERE PlayerTeam.playerId = $1")) } pub struct
GetTeamUserCharactersStmt(cornucopia_async::private::Stmt); impl GetTeamUserCharactersStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
playerid: &'a T1,) -> GetTeamUserCharactersQuery<'a,C,
GetTeamUserCharacters, 1>
{
    GetTeamUserCharactersQuery
    {
        client, params: [playerid,], stmt: &mut self.0, extractor:
        |row| { GetTeamUserCharactersBorrowed { id: row.get(0),playerid: row.get(1),archetypeid: row.get(2),experience: row.get(3),lvl: row.get(4),} }, mapper: |it| { <GetTeamUserCharacters>::from(it) },
    }
} }pub fn get_team_user_characters_with_stats() -> GetTeamUserCharactersWithStatsStmt
{ GetTeamUserCharactersWithStatsStmt(cornucopia_async::private::Stmt::new("SELECT Character.id AS character_id, Character.experience, ArchetypeStats.*
FROM PlayerTeam
INNER JOIN Character ON PlayerTeam.characterIDs @> ARRAY[Character.id] AND PlayerTeam.playerId = Character.playerID
INNER JOIN ArchetypeStats ON Character.archetypeID = ArchetypeStats.archetypeID AND Character.lvl = ArchetypeStats.lvl
WHERE PlayerTeam.playerId = $1")) } pub struct
GetTeamUserCharactersWithStatsStmt(cornucopia_async::private::Stmt); impl GetTeamUserCharactersWithStatsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
playerid: &'a T1,) -> GetTeamUserCharactersWithStatsQuery<'a,C,
GetTeamUserCharactersWithStats, 1>
{
    GetTeamUserCharactersWithStatsQuery
    {
        client, params: [playerid,], stmt: &mut self.0, extractor:
        |row| { GetTeamUserCharactersWithStatsBorrowed { character_id: row.get(0),experience: row.get(1),lvl: row.get(2),archetypeid: row.get(3),attack: row.get(4),defense: row.get(5),speed: row.get(6),hp: row.get(7),} }, mapper: |it| { <GetTeamUserCharactersWithStats>::from(it) },
    }
} }pub fn get_xp_required() -> GetXpRequiredStmt
{ GetXpRequiredStmt(cornucopia_async::private::Stmt::new("SELECT 
    requiredNextLevel 
from 
    ExperienceRequirement
where 
    lvl = $1")) } pub struct
GetXpRequiredStmt(cornucopia_async::private::Stmt); impl GetXpRequiredStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
lvl: &'a i16,) -> I32Query<'a,C,
i32, 1>
{
    I32Query
    {
        client, params: [lvl,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }}pub mod writes
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CreateCharacterParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,> { pub nftaddress: T1,pub playerid: T2,pub archetypeid: T3,}#[derive( Debug)] pub struct CreateArchetypeStatsParams<T1: cornucopia_async::StringSql,> { pub lvl: i16,pub archetypeid: T1,pub attack: i16,pub defense: i16,pub speed: i16,pub hp: i16,}#[derive( Debug)] pub struct UpdateCharaLvlXpParams<T1: cornucopia_async::StringSql,> { pub lvl: i16,pub xp: i32,pub charaid: T1,}#[derive( Debug)] pub struct AddGoldPlayerParams<T1: cornucopia_async::StringSql,> { pub delta_gold: i32,pub player_id: T1,}pub fn create_player() -> CreatePlayerStmt
{ CreatePlayerStmt(cornucopia_async::private::Stmt::new("insert into Player (id)
values ($1)")) } pub struct
CreatePlayerStmt(cornucopia_async::private::Stmt); impl CreatePlayerStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
playeraddress: &'a T1,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[playeraddress,]).await
} }pub fn create_character() -> CreateCharacterStmt
{ CreateCharacterStmt(cornucopia_async::private::Stmt::new("insert into Character (id, playerid, archetypeid, lvl, experience)
VALUES ($1, $2, $3, 1, 0)")) } pub struct
CreateCharacterStmt(cornucopia_async::private::Stmt); impl CreateCharacterStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
nftaddress: &'a T1,playerid: &'a T2,archetypeid: &'a T3,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[nftaddress,playerid,archetypeid,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CreateCharacterParams<T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CreateCharacterStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CreateCharacterParams<T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.nftaddress,&params.playerid,&params.archetypeid,)) }
}pub fn create_archetype_stats() -> CreateArchetypeStatsStmt
{ CreateArchetypeStatsStmt(cornucopia_async::private::Stmt::new("insert into archetypestats (lvl, archetypeid, attack, defense, speed, hp)
values ($1, $2, $3, $4, $5, $6)")) } pub struct
CreateArchetypeStatsStmt(cornucopia_async::private::Stmt); impl CreateArchetypeStatsStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
lvl: &'a i16,archetypeid: &'a T1,attack: &'a i16,defense: &'a i16,speed: &'a i16,hp: &'a i16,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[lvl,archetypeid,attack,defense,speed,hp,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CreateArchetypeStatsParams<T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CreateArchetypeStatsStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CreateArchetypeStatsParams<T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.lvl,&params.archetypeid,&params.attack,&params.defense,&params.speed,&params.hp,)) }
}pub fn update_chara_lvl_xp() -> UpdateCharaLvlXpStmt
{ UpdateCharaLvlXpStmt(cornucopia_async::private::Stmt::new("UPDATE character
set lvl = $1, experience = $2
where id = $3")) } pub struct
UpdateCharaLvlXpStmt(cornucopia_async::private::Stmt); impl UpdateCharaLvlXpStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
lvl: &'a i16,xp: &'a i32,charaid: &'a T1,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[lvl,xp,charaid,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpdateCharaLvlXpParams<T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpdateCharaLvlXpStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateCharaLvlXpParams<T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.lvl,&params.xp,&params.charaid,)) }
}pub fn add_gold_player() -> AddGoldPlayerStmt
{ AddGoldPlayerStmt(cornucopia_async::private::Stmt::new("UPDATE player
set gold = gold + $1
where id = $2")) } pub struct
AddGoldPlayerStmt(cornucopia_async::private::Stmt); impl AddGoldPlayerStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
delta_gold: &'a i32,player_id: &'a T1,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[delta_gold,player_id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, AddGoldPlayerParams<T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for AddGoldPlayerStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    AddGoldPlayerParams<T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.delta_gold,&params.player_id,)) }
}}}
