--! get_all_user_characters 
SELECT *
from Character
WHERE Character.playerId = :playerid;

--! get_all_user_characters_with_stats 
SELECT Character.id, Character.experience, ArchetypeStats.*
FROM Character
JOIN ArchetypeStats ON Character.archetypeID = ArchetypeStats.archetypeID
WHERE Character.playerId = :playerId;


--! get_team_user_characters
SELECT Character.*
FROM PlayerTeam
JOIN Character ON PlayerTeam.playerId = Character.playerID
WHERE PlayerTeam.playerId = :playerid;


--! get_team_user_characters_with_stats
SELECT Character.id AS character_id, Character.experience, ArchetypeStats.*
FROM PlayerTeam
INNER JOIN Character ON PlayerTeam.playerId = Character.playerID
INNER JOIN ArchetypeStats ON Character.archetypeID = ArchetypeStats.archetypeID
WHERE (PlayerTeam.playerId = :playerid and Character.lvl = ArchetypeStats.lvl);

--! get_xp_required 
SELECT 
    requiredNextLevel 
from 
    ExperienceRequirement
where 
    lvl = :lvl;