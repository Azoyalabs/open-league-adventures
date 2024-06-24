import { DB_CLIENT } from '$lib/client';
import type { LayoutLoad } from './$types';
import { init } from '@tma.js/sdk';
export const ssr = false;

export const load = (async () => {
	let userID = '';
	try {
		const { initData } = init();
		userID = initData!.user!.id.toString();
	} catch (error) {
		console.log('Error getting user ID');
	}

	const { data: playerPower } = await DB_CLIENT.from('playerpower')
		.select('*')
		.eq('playerid', userID)
		.single();

	return {
		userID: userID,
		playerPower: playerPower?.playerpower ?? 0
	};
}) satisfies LayoutLoad;
