use eternal_quest_sdk::types::Pawn;
use protodefs::pbfight::{fight_action::Action, FightActionAttack};
use rand::{Rng, RngCore};

use crate::actions::{ActionResult, ActionType, TickData, UnitReference};

pub struct FightInstance {
    player_chars: Vec<Pawn>,
    player_chars_hp_tracker: Vec<i16>,
    enemy_chars: Vec<Pawn>,
    enemy_chars_hp_tracker: Vec<i16>,
    player_accumulator: Vec<i16>,
    enemy_accumulator: Vec<i16>,
    rng: Box<dyn RngCore>,
}

enum DamageType {
    Physical,
    Magical,
}

impl FightInstance {
    pub fn new(player_chars: Vec<Pawn>, enemy_chars: Vec<Pawn>, rng: Box<dyn RngCore>) -> Self {
        let player_accumulator = vec![0; player_chars.len()];
        let enemy_accumulator = vec![0; enemy_chars.len()];

        let player_chars_hp_tracker = player_chars.iter().map(|chara| chara.hp).collect();
        let enemy_chars_hp_tracker = enemy_chars.iter().map(|chara| chara.hp).collect();

        return FightInstance {
            player_chars,
            player_chars_hp_tracker,
            enemy_chars,
            enemy_chars_hp_tracker,
            player_accumulator,
            enemy_accumulator,
            rng,
        };
    }

    /// Allow to continue from a previous fight if this is a multi fight battle
    pub fn new_with_hp(
        player_chars: Vec<Pawn>,
        player_chars_hp_tracker: Vec<i16>,
        enemy_chars: Vec<Pawn>,
        //enemy_chars_hp_tracker: Vec<i16>,
        rng: Box<dyn RngCore>,
    ) -> Self {
        let player_accumulator = vec![0; player_chars.len()];
        let enemy_accumulator = vec![0; enemy_chars.len()];

        let enemy_chars_hp_tracker = enemy_chars.iter().map(|chara| chara.hp).collect();

        return FightInstance {
            player_chars,
            player_chars_hp_tracker,
            enemy_chars,
            enemy_chars_hp_tracker,
            player_accumulator,
            enemy_accumulator,
            rng,
        };
    }

    pub fn tick(&mut self) -> Option<TickData> {
        // accumulate speed
        for id_chara in 0..self.player_chars.len() {
            self.player_accumulator[id_chara] += self.player_chars[id_chara].speed;
        }
        for id_chara in 0..self.enemy_chars.len() {
            self.enemy_accumulator[id_chara] += self.enemy_chars[id_chara].speed;
        }

        // find unit with highest accumulated
        let (id_active_unit, is_player_unit_active) = {
            let mut id_active_unit = 0;
            let mut is_player_unit_active = true;
            let mut active_unit_accumulated = 0;
            for id_chara in 0..self.player_chars.len() {
                if self.player_chars_hp_tracker[id_chara] > 0
                    && self.player_accumulator[id_chara] > active_unit_accumulated
                {
                    id_active_unit = id_chara;
                    is_player_unit_active = true;
                    active_unit_accumulated = self.player_accumulator[id_chara]
                }
            }
            for id_chara in 0..self.enemy_chars.len() {
                if self.enemy_chars_hp_tracker[id_chara] > 0
                    && self.enemy_accumulator[id_chara] > active_unit_accumulated
                {
                    id_active_unit = id_chara;
                    is_player_unit_active = false;
                    active_unit_accumulated = self.enemy_accumulator[id_chara]
                }
            }

            (id_active_unit, is_player_unit_active)
        };

        // now we know which unit is active
        // choose action depending on unit archetype
        let active_unit = if is_player_unit_active {
            &self.player_chars[id_active_unit]
        } else {
            &self.enemy_chars[id_active_unit]
        };

        let tick_data = match active_unit.archetypeid.as_str() {
            "warrior" => {
                // warriors only attack physical
                // find target at random
                let alive_ids = find_id_alives(
                    &self.player_chars_hp_tracker,
                    &self.enemy_chars_hp_tracker,
                    !is_player_unit_active,
                );

                // choose one at random
                let target_id = self.rng.gen_range(0..alive_ids.len());

                // compute damage
                let (attacker, defender) = if is_player_unit_active {
                    (
                        &self.player_chars[id_active_unit],
                        &self.enemy_chars[target_id],
                    )
                } else {
                    (
                        &self.enemy_chars[id_active_unit],
                        &self.player_chars[target_id],
                    )
                };
                let damage = compute_damage(attacker, defender, DamageType::Physical);

                // assign damage to target 
                let is_target_alive = if is_player_unit_active {
                    self.enemy_chars_hp_tracker[target_id] -= damage;
                    self.enemy_chars_hp_tracker[target_id] > 0
                } else {
                    self.player_chars_hp_tracker[target_id] -= damage;
                    self.player_chars_hp_tracker[target_id] > 0
                };

                TickData {
                    action_type: ActionType::Attack,
                    unit_acting: UnitReference {
                        is_player_unit: is_player_unit_active,
                        unit_id: id_active_unit,
                    },
                    action_results: vec![ActionResult::Damage {
                        target: UnitReference {
                            is_player_unit: !is_player_unit_active,
                            unit_id: target_id,
                        },
                        value: damage as usize,
                        unit_dies: !is_target_alive,
                    }],
                }
            }
            "ranger" => {
                panic!("ranger not implemented")
            }
            "white_mage" => {
                panic!("white_mage not implemented")
            }
            "black_mage" => {
                panic!("black_mage not implemented")
            }
            _ => {
                panic!("Unknown archetype id: {}", active_unit.archetypeid);
            }
        };

        // clear accumulator for this chara
        if is_player_unit_active {
            self.player_accumulator[id_active_unit] = 0;
        } else {
            self.enemy_accumulator[id_active_unit] = 0;
        }

        return Some(tick_data);
    }
}

fn find_id_alives(
    player_hps: &Vec<i16>,
    enemy_hps: &Vec<i16>,
    is_targetting_player: bool,
) -> Vec<usize> {
    let targets = if is_targetting_player {
        player_hps
    } else {
        enemy_hps
    };

    let mut id_alives = vec![];
    for i in 0..targets.len() {
        if targets[i] > 0 {
            id_alives.push(i);
        }
    }

    return id_alives;
}

fn compute_damage(attacker: &Pawn, defender: &Pawn, damage_type: DamageType) -> i16 {
    match damage_type {
        DamageType::Physical => {
            
        }
        DamageType::Magical => {
            panic!("Magical Damage not implemented");
        }
    }

    return 0;
}
