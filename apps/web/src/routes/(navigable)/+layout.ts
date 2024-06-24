import { DB_CLIENT } from '$lib/client';
import type { LayoutLoad } from './$types';

export const load = (async ({ parent }) => {
	const { userID } = await parent();
	const { data } = await DB_CLIENT.from('player').select('gold').eq('id', userID).single();
	return { gold: data?.gold || 0 };
}) satisfies LayoutLoad;
