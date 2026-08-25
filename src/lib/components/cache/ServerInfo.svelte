<script lang="ts">
	import type { InfoItem } from '$lib/types/cache';

	let { items }: { items: InfoItem[] } = $props();
</script>

{#if items.length > 0}
	<section class="info">
		<h3 class="section-title">服务器信息</h3>
		{#each items as item}
			<div class="kv-row">
				<span>{item.key}</span>
				<span>{item.value}</span>
			</div>
		{/each}
	</section>
{/if}

<style>
	.info {
		padding: 8px 16px 16px;
	}
</style>
