<script lang="ts">
	import { RankingCard } from 'ui';
	export let data;
	
</script>

<div class="max-h-full px-4 overflow-y-auto">
	<div>
		<h1 class="text-xl font-bold">Top players</h1>
		<div>Compete with others and reach the top!</div>
	</div>
	<div class="mt-4 space-y-4 mb-44">
		{#each data.players as player, i}
			<RankingCard value={{ name: player.playerid, rank: i + 1, power: player.playerpower }} />
		{/each}
	</div>
	<div class="absolute bottom-0 left-0 w-full p-3 bg-primary">
		<RankingCard
			value={{
				id: data.userID,
				name: data.userID,
				power: data.playerPower,
				rank: data.rank
			}}
		/>
	</div>
</div>
