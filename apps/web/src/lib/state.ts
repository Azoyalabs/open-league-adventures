import { browser, dev } from '$app/environment';
import { type Account } from '@tonconnect/ui';
import { get, writable } from 'svelte/store';

export const USER_ACCOUNT = writable<Account | null>(null);

if (browser && dev) {
	if (window) {
		globalThis.debug = () => {
			console.log(USER_ACCOUNT);
			console.log(get(USER_ACCOUNT));
		};
	}
}
