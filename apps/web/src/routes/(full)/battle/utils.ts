import type { BattleState, BattleAction } from '$lib/types/battle';
import type { StartFight, FightAction } from 'protos';

export function payloadToInitialState(startFight: StartFight): BattleState {
	const { enemyCharacters, fightId, playerCharacters } = startFight;
	const enemies = enemyCharacters.map((c) => {
		return {
			attack: c.attack,
			defense: c.defense,
			maxHP: c.maxHp,
			currentHP: c.maxHp,
			speed: c.speed
		};
	});
	const pawns = playerCharacters.map((c) => {
		return {
			attack: c.attack,
			defense: c.defense,
			maxHP: c.maxHp,
			currentHP: c.maxHp,
			speed: c.speed
		};
	});

	return {
		fightId,
		enemies,
		pawns,
		end: null
	};
}

export function payloadMessageTo(fightAction: FightAction): BattleAction {
	const { isPlayer, unitId } = fightAction.unitId!;

	const [actionResult] = fightAction.actionResult;

	const play: BattleAction = {
		origin: {
			from: isPlayer ? 'Player' : 'Enemy',
			unitId: unitId
		},
		target: {
			to: isPlayer ? 'Enemy' : 'Player',
			unitId: 0
		},
		result: {
			damage: 0,
			unitDies: false
		}
	};

	// TODO: we're only handling damage for now
	switch (actionResult.actionResultPayload.oneofKind) {
		case 'actionResultDamage':
			{
				const { unitDies, value } = actionResult.actionResultPayload.actionResultDamage;
				play.target.unitId = actionResult.target!.unitId;
				play.result.damage = value;
				play.result.unitDies = unitDies;
			}
			break;

		default:
			throw `Unimplemented action ${actionResult.actionResultPayload.oneofKind}`;
	}

	return play;
}
