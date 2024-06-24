<script lang="ts">
	import { BoxGeometry, MeshStandardMaterial } from 'three';
	import { T } from '@threlte/core';

	import StoneTile from './T_stonebrick_04.jpg?url';
    import Tile from './tiles.png?url';

    import { useTexture } from '@threlte/extras';
	import { RepeatWrapping } from 'three';

	const map = useTexture(Tile, {
		transform: (texture) => {
			texture.wrapS = RepeatWrapping;
			texture.wrapT = RepeatWrapping;
			texture.repeat.set(8, 8);
			return texture;
		}
	});
</script>

{#await map then value}
	<T.Mesh receiveShadow position={[0, 0, 0]} rotation.x={-Math.PI / 2}>
		<T.CircleGeometry  args={[6, 40]} />

		<T.MeshBasicMaterial map={value} />
	</T.Mesh>
{/await}
