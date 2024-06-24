<script lang="ts" context="module">
	export type Message = {
		characterID: string;
		action: 'attack' | 'die' | 'damage';
		damage?: number;
	};
</script>

<script lang="ts">
	import { T } from '@threlte/core';
	import { OrbitControls } from '@threlte/extras';

	import { writable } from 'svelte/store';
	import type { CharacterState } from './combat.types';

	let cameraPosition: [x: number, y: number, z: number] = [14, 20, 14];

	const POSITIONS = {
		ALLIED: [
			[-4, 0, -2],
			[-2, 0, -2],
			[0, 0, -2],
			[2, 0, -2]
		],
		OPPONENT: [
			[-4, 0, 2],
			[-2, 0, 2],
			[0, 0, 2],
			[2, 0, 2]
		]
	};

	export let opponents: (CharacterState & {
		model: 'Skeleton';
	})[];

	export let teamCharacters: (CharacterState & {
		model: 'Knight' | 'Mage';
	})[];

	import CSS2DScene from '$lib/components/app/Utils/CSS2DScene.svelte';
	import CharacterWithHp from '../../Combat/Character/CharacterWithHP.svelte';
	import { setContext } from 'svelte';
	import FightScene from './FightScene.svelte';

	teamCharacters.forEach((c, i) => {
		setContext(`ALLIED_${i}`, {
			position: POSITIONS.ALLIED[i],
			currentHP: writable(c.currentHP),
			maxHP: c.maxHP,
			model: c.model
		});
	});
	opponents.forEach((c, i) => {
		setContext(`OPPONENT_${i}`, {
			position: POSITIONS.OPPONENT[i],
			currentHP: writable(c.currentHP),
			maxHP: c.maxHP,
			model: c.model
		});
	});
</script>

<CSS2DScene>
	<T.DirectionalLight position={[0, 20, 20]} intensity={0.9} castShadow />
	<T.AmbientLight intensity={0.6} />

	<T.PerspectiveCamera makeDefault position={cameraPosition}>
		<OrbitControls enableDamping />
	</T.PerspectiveCamera>
	<!-- 
	<T.Mesh rotation.x={-Math.PI / 2} receiveShadow>
		<T.CircleGeometry args={[6, 40]} />
		<T.MeshStandardMaterial color="white" />
	</T.Mesh>
 -->

	<FightScene receiveShadow position={[0, -5, -20]}></FightScene>
	{#each teamCharacters as character, i}
		{#if character.currentHP > 0}
			<CharacterWithHp id={`ALLIED_${i}`}></CharacterWithHp>
		{/if}
	{/each}

	{#each opponents as character, i}
		{#if character.currentHP > 0}
			<CharacterWithHp rotation.y={Math.PI} id={`OPPONENT_${i}`}></CharacterWithHp>
		{/if}
	{/each}
</CSS2DScene>
