CREATE TABLE Clan (
    id int NOT NULL PRIMARY KEY,
    clanName TEXT NOT NULL,
    clanDescription TEXT NOT NULL,
    blason TEXT NOT NULL
);

CREATE TABLE CharacterArchetype (
  id varchar(256) NOT NULL PRIMARY KEY,
  archetypeName TEXT NOT NULL,
  archetypeDescription TEXT NOT NULL,
  model TEXT NOT NULL
);

CREATE TABLE ArchetypeStats (
  lvl smallint NOT NULL,
  archetypeID VARCHAR(256) NOT NULL references CharacterArchetype(id),
  
  attack smallint NOT NULL,
  defense smallint NOT NULL,
  speed smallint NOT NULL,
  hp smallint NOT NULL,
  /* tes autres stats */

  PRIMARY KEY(lvl, archetypeID)
);


CREATE TABLE Player (
    id varchar(256) NOT NULL PRIMARY KEY,
    clanID int REFERENCES Clan(id),
    gold int NOT NULL default 0
);

CREATE TABLE PlayerTeam (
  playerID varchar(256) NOT NULL references Player(id),
  id smallint NOT NULL, /*varchar(256) NOT NULL,*/
  characterIDs varchar(256)[4],
  
  PRIMARY KEY (playerID, id)
);

CREATE TABLE Character (
  id varchar(256) NOT NULL PRIMARY KEY,
  playerID varchar(256) NOT NULL references Player(id),
  archetypeID varchar(256) NOT NULL references CharacterArchetype(id),
  experience int NOT NULL,
  lvl smallint NOT NULL
);

CREATE TABLE ExperienceRequirement (
    lvl smallint NOT NULL PRIMARY KEY,
    requiredNextLevel int NOT NULL
);




