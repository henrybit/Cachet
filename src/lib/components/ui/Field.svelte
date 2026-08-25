<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		label: string;
		hint?: string;
		children: Snippet;
	};

	let { label, hint, children }: Props = $props();
</script>

<div class="field">
	<span>{label}</span>
	{@render children()}
	{#if hint}<p class="hint">{hint}</p>{/if}
</div>
