use protodefs::pbfight::{self, FightAction};

#[derive(Debug)]
pub enum ActionType {
    Attack,
    Skill { skill_id: usize },
    Spell { spell_id: usize },
}

#[derive(Debug)]
pub enum ActionResult {
    Miss {
        target: UnitReference,
    },
    Damage {
        target: UnitReference,
        value: usize,
        unit_dies: bool,
    },
    Heal {
        target: UnitReference,
        value: usize,
    },
}

impl ActionResult {
    pub fn to_protobuf(self) -> protodefs::pbfight::ActionResult {
        match self {
            ActionResult::Damage { target, value, unit_dies } => {
                protodefs::pbfight::ActionResult {
                    target: Some(target.to_protobuf()),
                    action_result_payload: Some(
                        protodefs::pbfight::action_result::ActionResultPayload::ActionResultDamage(
                            protodefs::pbfight::ActionResultDamage {
                                value: value as u32,
                                unit_dies,
                            },
                        ),
                    ),
                }
            },
            ActionResult::Heal { target, value } => {
                protodefs::pbfight::ActionResult {
                    target: Some(target.to_protobuf()),
                    action_result_payload: Some(
                        protodefs::pbfight::action_result::ActionResultPayload::ActionResultHeal(
                            protodefs::pbfight::ActionResultHeal {
                                value: value as u32,
                            },
                        ),
                    ),
                }
            },
            ActionResult::Miss { target } => {
                protodefs::pbfight::ActionResult {
                    target: Some(target.to_protobuf()),
                    action_result_payload: Some(
                        protodefs::pbfight::action_result::ActionResultPayload::ActionResultMiss(
                            protodefs::pbfight::ActionResultMiss {
                            },
                        ),
                    ),
                }
            }
        }
    }

}


#[derive(Debug)]
pub struct UnitReference {
    pub is_player_unit: bool,
    pub unit_id: usize,
}

impl UnitReference {
    pub fn to_protobuf(self) -> pbfight::UnitReference {
        pbfight::UnitReference {
            is_player: self.is_player_unit,
            unit_id: self.unit_id as u32
        }
    }

}



#[derive(Debug)]
pub struct TickData {
    pub action_type: ActionType,
    pub unit_acting: UnitReference,
    pub action_results: Vec<ActionResult>,
}

impl TickData {
    pub fn to_protobuf(self) -> FightAction {
        FightAction {
            unit_id: Some(self.unit_acting.to_protobuf()),
            action: Some(protodefs::pbfight::fight_action::Action::FightActionAttack(
                protodefs::pbfight::FightActionAttack {},
            )),
            action_result: self.action_results.into_iter().map(|elem| elem.to_protobuf()).collect(),
        }
    }
}
