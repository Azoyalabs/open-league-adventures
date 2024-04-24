--! create_player 
insert into Player (id)
values (:playeraddress);


--! create_character
insert into Character (id, playerid, archetypeid, lvl, experience)
VALUES (:nftaddress, :playerid, :archetypeid, 1, 0);