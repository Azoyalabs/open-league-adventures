--! create_player 
insert into Player (id)
values (:playeraddress);


--! create_character
insert into Character (id, playerid, archetypeid, lvl, experience)
VALUES (:nftaddress, :playerid, :archetypeid, 1, 0);


--! create_archetype_stats
insert into archetypestats (lvl, archetypeid, attack, defense, speed, hp)
values (:lvl, :archetypeid, :attack, :defense, :speed, :hp);