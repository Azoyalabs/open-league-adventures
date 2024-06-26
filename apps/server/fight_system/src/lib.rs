use game_types::Character;
use rand::prelude::*;

pub mod actions;
pub mod fight_v2;

use actions::{ActionResult, ActionType, TickData, UnitReference};

pub enum FightStatus {
    Ongoing,
    FightEnded { player_won: bool },
}

pub fn tick(
    player_chars: &mut Vec<Character>,
    enemy_chars: &mut Vec<Character>,
    player_accumulator: &mut Vec<i32>,
    enemy_accumulator: &mut Vec<i32>,
    rng: &mut dyn RngCore,
) -> (TickData, FightStatus) {
    for (id, chara) in player_chars.iter().enumerate() {
        if chara.current_hp > 0 {
            player_accumulator[id] += chara.speed as i32;
        }
    }

    for (id, chara) in enemy_chars.iter().enumerate() {
        if chara.current_hp > 0 {
            enemy_accumulator[id] += chara.speed as i32;
        }
    }

    // find chara with highest speed
    let (id_chara, is_player) = find_chara_highest_speed(player_accumulator, enemy_accumulator);

    // choose action

    // process action
    // just remove 1 hp for now to random unit
    let id_target: usize = {
        let (target_range, chars) = match is_player {
            true => (0..enemy_chars.len(), &enemy_chars),
            false => (0..player_chars.len(), &player_chars),
        };

        let mut target_id: usize; // = 0;
        loop {
            // choose target
            target_id = rng.gen_range(target_range.clone());
            // check is not dead
            if chars[target_id].current_hp > 0 {
                break;
            }
        }

        target_id
    };

    if is_player {
        enemy_chars[id_target].current_hp -= 1;
    } else {
        player_chars[id_target].current_hp -= 1;
    }


    let is_target_dead = if is_player {
        enemy_chars[id_target].current_hp == 0
    } else {
        player_chars[id_target].current_hp == 0
    };

    let action = TickData {
        action_type: ActionType::Attack,
        unit_acting: UnitReference {
            is_player_unit: is_player,
            unit_id: id_chara,
        },
        action_results: vec![ActionResult::Damage {
            target: UnitReference {
                is_player_unit: !is_player,
                unit_id: id_target,
            },
            value: 1,
            unit_dies: is_target_dead,
        }],
    };

    // remove speed
    // just clear it tbh?
    if is_player {
        player_accumulator[id_chara] = 0;
    } else {
        enemy_accumulator[id_chara] = 0;
    }

    // check if all been wiped
    let fight_status = check_fight_status(player_chars, enemy_chars);

    (action, fight_status)
}


fn check_fight_status(player_chars: &Vec<Character>, enemy_chars: &Vec<Character>) -> FightStatus {
    let mut player_all_dead: bool = true;
    let mut enemy_all_dead: bool = true;

    for chara in player_chars.iter() {
        if chara.current_hp > 0 {
            player_all_dead = false;
            break;
        }
    }

    for chara in enemy_chars.iter() {
        if chara.current_hp > 0 {
            enemy_all_dead = false;
            break;
        }
    }

    if player_all_dead {
        FightStatus::FightEnded { player_won: false }
    } else if enemy_all_dead {
        return FightStatus::FightEnded { player_won: true };
    } else {
        return FightStatus::Ongoing;
    }
}

/// returns id of chara and is_player
fn find_chara_highest_speed(
    player_accumulator: &Vec<i32>,
    enemy_accumulator: &Vec<i32>,
) -> (usize, bool) {
    let mut highest_speed = -999;
    let mut chosen_id: usize = 0;
    let mut is_player = true;

    for (id, acc_speed) in player_accumulator.iter().enumerate() {
        if player_accumulator[id] > highest_speed {
            highest_speed = *acc_speed;
            chosen_id = id;
        }
    }

    for (id, acc_speed) in enemy_accumulator.iter().enumerate() {
        if enemy_accumulator[id] > highest_speed {
            highest_speed = *acc_speed;
            chosen_id = id;
            is_player = false;
        }
    }

    (chosen_id, is_player)
}

/*
fn process_action(
    player_chars: &mut Vec<Character>,
    enemy_chars: &mut Vec<Character>,
    action: Actions,
) {
}
*/