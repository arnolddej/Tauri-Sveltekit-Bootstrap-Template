<script lang="ts">
	import Greet from '$lib/Greet.svelte';
	import Serialport from '$lib/Serialport.svelte';
	// import { Button, Navbar } from '@sveltestrap/sveltestrap';
	// import { Styles } from '@sveltestrap/sveltestrap';

	let m = { x: 0, y: 0 };

	function handleMove(event: any) {
		m.x = event.clientX;
		m.y = event.clientY;
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h1>
	Create the top menu bar (left or top) and then create tabs under it or on top. Next create a
	footer with a light dark mode toggler and a few status indicators Also try adding icons from
	bootstrap or another source
</h1>

<div on:pointermove={handleMove}>
	The pointer is at {m.x} x {m.y}
</div>

<Greet />

<span> </span>

<Serialport />

<h2>The router works, but i have to find out the [slug] part.</h2>

<style>
	h2 {
		font-size: small;
	}

	/* div {
		position: fixed;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		padding: 1rem;
	} */
</style>
