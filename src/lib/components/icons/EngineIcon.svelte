<script lang="ts">
	import MemcachedIcon from '$lib/components/icons/MemcachedIcon.svelte';
	import RedisIcon from '$lib/components/icons/RedisIcon.svelte';
	import type { CacheEngine } from '$lib/types/cache';

	let { engine, size = 28 }: { engine: CacheEngine; size?: number } = $props();
	const iconSize = $derived(Math.round(size * 0.56));
</script>

<span class="engine-icon {engine}" style="width: {size}px; height: {size}px;" title={engine}>
	{#if engine === 'redis'}
		<RedisIcon size={iconSize} />
	{:else}
		<MemcachedIcon size={iconSize} />
	{/if}
</span>
