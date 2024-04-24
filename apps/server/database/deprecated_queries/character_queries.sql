--! create_character
INSERT INTO
    Characters (
        Ownerid,
        Lvl,
        Experience,
        Strength,
        Speed,
        IsTeamMember
    )
VALUES
    (
        :owner_id,
        :lvl,
        :experience,
        :strength,
        :speed,
        :is_team_member
    );

--! user_characters 
SELECT
    Id,
    Lvl,
    Experience,
    Strength,
    Speed
FROM
    Characters
WHERE
    Ownerid = :owner_id;

--! user_team_characters
SELECT
    Id,
    Lvl,
    Experience,
    Strength,
    Speed
FROM
    Characters
WHERE
    Ownerid = :owner_id
    AND IsTeamMember = true;

--! update_chara_xp
UPDATE
    Characters
SET
    Experience = :experience
WHERE
    Id = :unit_id;


--! get_all_user_characters 
SELECT
    *
from
    Player as p
    INNER JOIN Character as c on c.playerID = p.id
    INNER JOIN CharacterArchetype as ca on ca.id = c.id
    INNER JOIN ArchetypeStats as archstats on archstats.achetypeID = ca.id
    AND ca.level = c.level
where
    id = :player_id,
order by
    c.level;