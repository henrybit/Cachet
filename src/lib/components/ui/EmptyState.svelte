<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		title: string;
		description?: string;
		children?: Snippet;
	};

	let { title, description, children }: Props = $props();
</script>

<div class="empty">
	<strong>{title}</strong>
	{#if description}<p class="muted">{description}</p>{/if}
	{#if children}{@render children()}{/if}
</div>
