--! set_experience 
INSERT INTO
    ExperienceRequirement(lvl, requiredNextLevel)
VALUES
    (:lvl, :experience);

