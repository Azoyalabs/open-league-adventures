<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Feather from '$lib/assets/itemicon_feather.png';
	import Sword from '$lib/assets/Itemicon_Equipment_Sword.png';
	import Star from '$lib/assets/Icon_RankIcon_Star01_l.png';
	import Shield from '$lib/assets/shield.png';
	import Health from '$lib/assets/Icon_ColorIcon_Life.png';

	export let hero: {
		id: string;
		name: string;
		level: number;
		rarity: number;
		image: string;
		available: boolean;
		class: string;
		health: number;
		attack: number;
		defense: number;
		speed: number;
		fielded: boolean;
		experience: number;
		toNextLevel: number;
	};

	const statsKeys = ['health', 'attack', 'defense', 'speed'];
	const stats = statsKeys.map((key) => ({ key, value: hero[key] as number }));

	const statToIcon = {
		health: Health,
		attack: Sword,
		defense: Shield,
		speed: Feather
	};

	export let action: (id: string) => void;
	export let disabled = false;

	$: experienceProgress = (hero.experience / hero.toNextLevel) * 100;
	$: tnl = hero.toNextLevel - hero.experience;
</script>

<div class="mx-auto w-full max-w-[422px] space-y-4 p-4">
	<div class="grid grid-cols-3">
		<div class="col-span-2">
			<div class="text-lg font-medium">
				{hero.class}
			</div>
			<div class="mt-1 flex items-center space-x-0.5">
				{#each new Array(5) as _, i}
					{@const isRare = hero.rarity > i}
					<img src={Star} alt="" class="h-4 w-4 {isRare ? '' : 'grayscale'} " />
				{/each}
			</div>
		</div>
		<div class="space-y-1 text-right">
			<div>
				Level <span class="text-xl font-semibold text-yellow-500">{hero.level}</span>
			</div>
			<div>
				<div
					class="w-full h-2 border border-yellow-500 rounded-lg from-yellow-500"
					style="background-image: linear-gradient(to right, var(--tw-gradient-from), var(--tw-gradient-from) {experienceProgress}%, transparent {experienceProgress +
						1}%);"
				></div>

				
			</div>
			<div class="text-xs">
				<span class="text-yellow-500">
					{tnl}
				</span> exp remaining
			</div>
		</div>
	</div>
	<div class="">
		<div class="flex items-baseline justify-between">
			<div class="text-xs font-medium uppercase text-muted-foreground">Stats</div>
		</div>
		<div class="grid grid-cols-2 gap-4 mt-2">
			{#each stats as { key, value }}
				<div class="flex items-center px-4 py-2 space-x-3 border rounded-lg bg-muted">
					<div>
						<img src={statToIcon[key]} alt={key} class="w-6 h-6" />
					</div>
					<div class="font-semibold">
						<div class="text-xs uppercase text-muted-foreground">{key}</div>
						<div class="text-yellow-500">{value}</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
	<div class="">
		<Button class="w-full" on:click={() => action(hero.id)} {disabled}
			>{hero.fielded ? 'Remove' : 'Use'}</Button
		>
	</div>
</div>
