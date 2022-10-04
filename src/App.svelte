<script lang="ts">
	import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
	import { emit, Event, listen } from "@tauri-apps/api/event";

	var games = [];

	invoke("find_games").then((_:any)=>games = _);
	function launch_game(game) {
		invoke("launch_game", {
			appid:game.appid
		});
	}
</script>

<main>
	<div id = "grid">
		{#each games as game}
			<div class = "game">
				<!-- <p class = "text-slate-100">{game.name}</p> -->
			<img alt = {game.name} src = {convertFileSrc(game.header_path)}>
			<!-- <img alt = {game.name} src = {convertFileSrc(game.icon_path)}> -->
			<!-- <button on:click={() => launch_game(game)}>launch</button> -->
			</div>

		{/each}
	</div>
</main>
<style>
	#grid{
		display:grid;
		grid-gap:1em;
		grid-template-columns: fit-content(8ch) auto;
		/* grid-auto-rows: auto; */
		/* grid-auto-columns: auto; */
	}
</style>
