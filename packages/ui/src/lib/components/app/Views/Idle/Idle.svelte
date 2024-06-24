<script lang="ts">
	import { Canvas } from '@threlte/core';
	import { T } from '@threlte/core';
	import { injectLookAtPlugin } from '$lib/threlte.utils';
	import { Button } from '$lib/components/ui/button';
	import Energy from '$lib/assets/Itemicon_Energy_0_Yellow.png';
	import CharacterWrapper, { type AllowedModels } from '$lib/assets/Models/CharacterWrapper.svelte';
	import Ground from './Ground.svelte';
	import { Environment } from '@threlte/extras';

	injectLookAtPlugin();

	import { Sky } from '@threlte/extras';
	import { MeshBasicMaterial } from 'three';
	let cameraPosition: [x: number, y: number, z: number] = [10, 3, 10];
	let lookTarget = cameraPosition;

	export let models: AllowedModels[] = [];

	const modelPositions = [
		[1, 0, -3],
		[-1, 0, 0],
		[1, 0, 3],
		[4, 0, 0]
	];
</script>

<div class="relative h-full border me">
	<Canvas>

		<T.DirectionalLight position={[0, 10, 10]} intensity={0.6} castShadow />
		<T.AmbientLight intensity={0.8} />
		<!-- 
		<T.Mesh receiveShadow position={[0, -0.5, 0]} rotation.x={-Math.PI / 2}>
			<T.CircleGeometry args={[6, 40]} />

			<T.MeshBasicMaterial />
		</T.Mesh> -->
		 <Ground></Ground> 
		<T.PerspectiveCamera
			makeDefault
			position={cameraPosition}
			on:create={({ ref }) => {
				ref.lookAt(0, 1, 0);
			}}
		></T.PerspectiveCamera>

		{#each models as model, i}
			<CharacterWrapper {model} lookAt={lookTarget} position={modelPositions[i]} ID="model-{i}"
			></CharacterWrapper>
		{/each}
	</Canvas>
	<div class="absolute z-10 flex flex-col items-center w-full bottom-10">
		<div class="flex flex-col items-center text-center font-fredoka">
			<div class="text-xs font-bold uppercase">Act 1</div>
			<div class="text-2xl font-semibold tracking-wider uppercase">The Test</div>
		</div>
		<Button href="/battle" class="flex items-center mx-auto mt-6 h-fit">
			<img src={Energy} alt="Energy" class="mr-3 h-9 w-9" />
			<div class="text-left">
				<div class="text-lg">Play</div>
				<div>0 energy</div>
			</div>
		</Button>
	</div>
</div>

<style>
	.me {
		background-color: rgb(14 22 37 / 1);
	}
</style>
