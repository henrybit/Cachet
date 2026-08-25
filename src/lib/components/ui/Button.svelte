<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'primary' | 'ghost' | 'danger' | 'default' | 'icon';

	type Props = {
		variant?: Variant;
		type?: 'button' | 'submit';
		disabled?: boolean;
		loading?: boolean;
		onclick?: (event: MouseEvent) => void;
		title?: string;
		children: Snippet;
	};

	let {
		variant = 'default',
		type = 'button',
		disabled = false,
		loading = false,
		onclick,
		title,
		children
	}: Props = $props();
</script>

<button class="btn {variant}" {type} {title} disabled={disabled || loading} {onclick}>
	{#if loading}<span class="spinner"></span>{/if}
	{@render children()}
</button>
