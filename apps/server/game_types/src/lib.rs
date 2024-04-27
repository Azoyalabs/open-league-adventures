use serde::{Deserialize, Serialize};

/// Character data queried from database
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CharacterRaw {
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub experience: u32,
}

impl CharacterRaw {
    pub fn to_character(&self) -> Character {
        Character {
            max_hp: self.max_hp,
            current_hp: self.max_hp,
            attack: self.attack,
            defense: self.defense,
            speed: self.speed,
        }
    }
}

pub struct CharacterRawBuilder {
    inner: CharacterRaw
}

impl Default for CharacterRawBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterRawBuilder {
    pub fn new() -> Self {
        CharacterRawBuilder { inner: CharacterRaw { max_hp: 0, attack: 0, defense: 0, speed: 0, experience: 0 } }
    }

    pub fn hp(mut self, value: u32) -> Self {
        self.inner.max_hp = value;
        self
    }

    pub fn attack(mut self, value: u32) -> Self {
        self.inner.attack = value;
        self
    }

    pub fn defense(mut self, value: u32) -> Self {
        self.inner.defense = value;
        self
    }

    pub fn speed(mut self, value: u32) -> Self {
        self.inner.speed = value;
        self
    }

    pub fn build(self) -> CharacterRaw {
        self.inner
    }
}


/// Character data used in game  
pub struct Character {
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
}
