--! create_player 
insert into Player (id)
values (:playeraddress);


--! create_character
insert into Character (id, playerid, archetypeid, lvl, experience)
VALUES (:nftaddress, :playerid, :archetypeid, 1, 0);


--! create_archetype_stats
insert into archetypestats (lvl, archetypeid, attack, defense, speed, hp)
values (:lvl, :archetypeid, :attack, :defense, :speed, :hp);

--! update_chara_lvl_xp
UPDATE character
set lvl = :lvl, experience = :xp
where id = :charaid;

--! add_gold_player 
UPDATE player
set gold = gold + :delta_gold
where id = :player_id;