<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import '$lib/styles/app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>Cached GUI</title>
	<link rel="icon" href={favicon} />
</svelte:head>

<div id="app-root">
	{@render children()}
</div>
