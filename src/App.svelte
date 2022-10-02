<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { emit, Event, listen } from "@tauri-apps/api/event";

	var games = [];

	invoke("find_games").then((_:any)=>games = _);
	function launch_game(game) {
		invoke("launch_game", {
			appid:game.appid
		});
	}
	window["x"] = this;
</script>

<main>
	{#each games as game}
		<p>{game.name}</p>
		<button on:click={() => launch_game(game)}>launch</button>
	{/each}
</main>

<style>
</style>
