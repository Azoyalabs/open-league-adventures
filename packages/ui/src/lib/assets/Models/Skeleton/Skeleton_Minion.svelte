<!--
Auto-generated by: https://github.com/threlte/threlte/tree/main/packages/gltf
Command: npx @threlte/gltf@2.0.3 ./models/skeleton_minion/Skeleton_Minion.glb --types --shadows --transform --simplify
-->
<script lang="ts" context="module">
	let id = 0;
</script>

<script lang="ts">
	import type * as THREE from 'three';
	import { Group } from 'three';
	import { T, type Props, type Events, type Slots, forwardEventHandlers } from '@threlte/core';
	import { useGltf, useGltfAnimations } from '@threlte/extras';

	type $$Props = Props<THREE.Group>;
	type $$Events = Events<THREE.Group>;
	type $$Slots = Slots<THREE.Group> & { fallback: {}; error: { error: any } };

	export const ref = new Group();

	type ActionName =
		| '1H_Melee_Attack_Chop'
		| '1H_Melee_Attack_Jump_Chop'
		| '1H_Melee_Attack_Slice_Diagonal'
		| '1H_Melee_Attack_Slice_Horizontal'
		| '1H_Melee_Attack_Stab'
		| '1H_Ranged_Aiming'
		| '1H_Ranged_Reload'
		| '1H_Ranged_Shoot'
		| '1H_Ranged_Shooting'
		| '2H_Melee_Attack_Chop'
		| '2H_Melee_Attack_Slice'
		| '2H_Melee_Attack_Spin'
		| '2H_Melee_Attack_Spinning'
		| '2H_Melee_Attack_Stab'
		| '2H_Melee_Idle'
		| '2H_Ranged_Aiming'
		| '2H_Ranged_Reload'
		| '2H_Ranged_Shoot'
		| '2H_Ranged_Shooting'
		| 'Block'
		| 'Block_Attack'
		| 'Block_Hit'
		| 'Blocking'
		| 'Cheer'
		| 'Death_A'
		| 'Death_A_Pose'
		| 'Death_B'
		| 'Death_B_Pose'
		| 'Death_C_Pose'
		| 'Death_C_Skeletons'
		| 'Death_C_Skeletons_Resurrect'
		| 'Dodge_Backward'
		| 'Dodge_Forward'
		| 'Dodge_Left'
		| 'Dodge_Right'
		| 'Dualwield_Melee_Attack_Chop'
		| 'Dualwield_Melee_Attack_Slice'
		| 'Dualwield_Melee_Attack_Stab'
		| 'Hit_A'
		| 'Hit_B'
		| 'Idle'
		| 'Idle_B'
		| 'Idle_Combat'
		| 'Interact'
		| 'Jump_Full_Long'
		| 'Jump_Full_Short'
		| 'Jump_Idle'
		| 'Jump_Land'
		| 'Jump_Start'
		| 'Lie_Down'
		| 'Lie_Idle'
		| 'Lie_Pose'
		| 'Lie_StandUp'
		| 'PickUp'
		| 'Running_A'
		| 'Running_B'
		| 'Running_C'
		| 'Running_Strafe_Left'
		| 'Running_Strafe_Right'
		| 'Sit_Chair_Down'
		| 'Sit_Chair_Idle'
		| 'Sit_Chair_Pose'
		| 'Sit_Chair_StandUp'
		| 'Sit_Floor_Down'
		| 'Sit_Floor_Idle'
		| 'Sit_Floor_Pose'
		| 'Sit_Floor_StandUp'
		| 'Skeleton_Inactive_Standing_Pose'
		| 'Skeletons_Awaken_Floor'
		| 'Skeletons_Awaken_Floor_Long'
		| 'Skeletons_Awaken_Standing'
		| 'Skeletons_Inactive_Floor_Pose'
		| 'Spawn_Air'
		| 'Spawn_Ground'
		| 'Spawn_Ground_Skeletons'
		| 'Spellcast_Long'
		| 'Spellcast_Raise'
		| 'Spellcast_Shoot'
		| 'Spellcast_Summon'
		| 'Spellcasting'
		| 'T-Pose'
		| 'Taunt'
		| 'Taunt_Longer'
		| 'Throw'
		| 'Unarmed_Idle'
		| 'Unarmed_Melee_Attack_Kick'
		| 'Unarmed_Melee_Attack_Punch_A'
		| 'Unarmed_Melee_Attack_Punch_B'
		| 'Unarmed_Pose'
		| 'Use_Item'
		| 'Walking_A'
		| 'Walking_B'
		| 'Walking_Backwards'
		| 'Walking_C'
		| 'Walking_D_Skeletons';
	type GLTFResult = {
		nodes: {
			Skeleton_Minion_ArmLeft: THREE.SkinnedMesh;
			Skeleton_Minion_ArmRight: THREE.SkinnedMesh;
			Skeleton_Minion_Body: THREE.SkinnedMesh;
			Skeleton_Minion_Cloak: THREE.SkinnedMesh;
			Skeleton_Minion_Eyes: THREE.SkinnedMesh;
			Skeleton_Minion_Head: THREE.SkinnedMesh;
			Skeleton_Minion_Jaw: THREE.SkinnedMesh;
			Skeleton_Minion_LegLeft: THREE.SkinnedMesh;
			Skeleton_Minion_LegRight: THREE.SkinnedMesh;
			root: THREE.Bone;
		};
		materials: {
			skeleton: THREE.MeshStandardMaterial;
			Glow: THREE.MeshStandardMaterial;
		};
	};
	import { models } from 'shared';

	import { onMount } from 'svelte';
//	const gltf = useGltf(`${models.skeleton}?${id++}`, { useDraco: true });
const gltf = useGltf(`${models["./Skeleton_Minion-transformed.glb"]}?${id++}`, { useDraco: true });

/*
	const gltf = useGltf<GLTFResult>(`./Skeleton_Minion-transformed.glb?url}?${id++}`, {
		useDraco: true
	});
	*/
	export const { actions, mixer } = useGltfAnimations<ActionName>(gltf, ref);

	onMount(() => {
		return () => {
			id--;
		};
	});
	const component = forwardEventHandlers();
</script>

<T is={ref} dispose={false} {...$$restProps} bind:this={$component}>
	{#await gltf}
		<slot name="fallback" />
	{:then gltf}
		<T.Group name="Scene">
			<T.Group name="Rig">
				<T is={gltf.nodes.root} />
				<T.SkinnedMesh
					name="Skeleton_Minion_ArmLeft"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_ArmLeft.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_ArmLeft.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_ArmRight"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_ArmRight.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_ArmRight.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_Body"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_Body.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_Body.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_Cloak"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_Cloak.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_Cloak.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_Eyes"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_Eyes.geometry}
					material={gltf.materials.Glow}
					skeleton={gltf.nodes.Skeleton_Minion_Eyes.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_Head"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_Head.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_Head.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_Jaw"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_Jaw.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_Jaw.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_LegLeft"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_LegLeft.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_LegLeft.skeleton}
				/>
				<T.SkinnedMesh
					name="Skeleton_Minion_LegRight"
					castShadow
					receiveShadow
					geometry={gltf.nodes.Skeleton_Minion_LegRight.geometry}
					material={gltf.materials.skeleton}
					skeleton={gltf.nodes.Skeleton_Minion_LegRight.skeleton}
				/>
			</T.Group>
		</T.Group>
	{:catch error}
		<slot name="error" {error} />
	{/await}

	<slot {ref} />
</T>
