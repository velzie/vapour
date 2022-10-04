import App from './App.svelte';
import "./dist.css";
const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;
