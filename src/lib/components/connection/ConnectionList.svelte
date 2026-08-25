<script lang="ts">
	import ConnectionItem from '$lib/components/connection/ConnectionItem.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import type { ConnectionConfig } from '$lib/types/cache';

	type Props = {
		items: ConnectionConfig[];
		selectedId: string | null;
		query: string;
		onSelect: (id: string) => void;
		onEdit: (connection: ConnectionConfig) => void;
		onDelete: (id: string) => void;
	};

	let { items, selectedId, query, onSelect, onEdit, onDelete }: Props = $props();

	const filtered = $derived.by(() => {
		const keyword = query.trim().toLowerCase();
		if (!keyword) return items;
		return items.filter((item) =>
			[item.name, item.host, item.engine, String(item.port)].join(' ').toLowerCase().includes(keyword)
		);
	});
</script>

{#if filtered.length === 0}
	<EmptyState title="没有匹配的连接" description="换个关键词，或新建 Redis / Memcached 连接。" />
{:else}
	<div class="scroll">
		{#each filtered as connection (connection.id)}
			<ConnectionItem
				{connection}
				active={connection.id === selectedId}
				{onSelect}
				{onEdit}
				{onDelete}
			/>
		{/each}
	</div>
{/if}
