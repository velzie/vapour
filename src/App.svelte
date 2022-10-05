<script lang="ts">
	import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
	import { emit, Event, listen } from "@tauri-apps/api/event";
	import { appWindow } from "@tauri-apps/api/window";

	var games = [];

	invoke("find_games").then((_: any) => (games = _));
	function launch_game(game) {
		invoke("launch_game", {
			appid:game.appid
		});
	}
</script>

<main class = "window flex flex-col">

	<div class="title-bar">
		<div class="title-bar-text">Vapour</div>
		<div class="title-bar-controls">
			<button aria-label="Minimize" on:click={appWindow.minimize} />
			<button aria-label="Maximize" on:click={appWindow.maximize} />
			<button aria-label="Close" on:click={appWindow.close} />
		</div>
	</div>
	<div id="grid">
		{#each games as game}
			<div class="game window" on:click={() => launch_game(game)}>
				<div class = "flex justify-center p-3">
					<img class = "status-bar-field gameimg" alt={game.name} src={convertFileSrc(game.header_path)} on:error = {function () {this.src = "../clouds.png"}}/>
				</div>
				<!-- <img alt = {game.name} src = {convertFileSrc(game.icon_path)}> -->
				<!-- <button on:click={() => launch_game(game)}>launch</button> -->
				<div class="gamefooter flex items-center justify-center">
					<p class="text text-lg">{game.name}</p>
				</div>
			</div>
		{/each}
	</div>
</main>

<style>

	.game {
		height: min-content;
		/* text-size-adjust: 80%; */
		white-space: nowrap;
		text-overflow: clip;
	}
	main {
		overflow: hidden;
		height: 100%;
	}
	.title-bar {
		background: linear-gradient(90deg, rgb(27, 121, 188), #d010d0);
	}
	#grid {
		background-color: pink;
		overflow-y: scroll;
		overflow-x: hidden;
		height: 100%;
		display: grid;
		grid-gap:1.5em;
		padding: 1.5em;
		grid-auto-rows: min-content;
		grid-template-columns: repeat(auto-fill,minmax(20vw, 1fr));
		background-image: url("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn.wallpapersafari.com%2F95%2F75%2Fxwho2M.jpg&f=1&nofb=1&ipt=e7da74d9e19457455c3a5d85ab40227032389845f2cf7ee570aba33e3886a7d7&ipo=images");
		/* filter:hue-rotate(45deg) */
	}
	.gameimg {
		/* width: 20vw; */
		/* height: auto; */
	}
</style>
