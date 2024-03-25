<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let greetMsg = '';

	async function getSerialPorts() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		greetMsg = await invoke('list_available_serialports');
		console.log(greetMsg);
		console.log(typeof greetMsg);
	}
</script>

<div>
	<form class="row" on:submit|preventDefault={getSerialPorts}>
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
</div>

<!-- tty0tty is a serial port emulator thing -->
