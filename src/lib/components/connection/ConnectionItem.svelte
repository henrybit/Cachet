<script lang="ts">
	import EngineIcon from '$lib/components/icons/EngineIcon.svelte';
	import { ENGINE_META, formatEndpoint } from '$lib/config/engines';
	import type { ConnectionConfig } from '$lib/types/cache';

	type Props = {
		connection: ConnectionConfig;
		active?: boolean;
		onSelect: (id: string) => void;
		onEdit: (connection: ConnectionConfig) => void;
		onDelete: (id: string) => void;
	};

	let { connection, active = false, onSelect, onEdit, onDelete }: Props = $props();
	const meta = $derived(ENGINE_META[connection.engine]);
</script>

<div class="row">
	<button class="list-item" class:active type="button" onclick={() => onSelect(connection.id)}>
		<EngineIcon engine={connection.engine} size={32} />
		<div class="item-copy">
			<strong>{connection.name}</strong>
			<span>{formatEndpoint(connection.host, connection.port)}</span>
		</div>
		<span class="badge {connection.engine}">{meta.shortLabel}</span>
	</button>
	<div class="actions">
		<button class="btn ghost icon" type="button" title="编辑连接" onclick={() => onEdit(connection)}>✎</button>
		<button class="btn ghost icon" type="button" title="删除连接" onclick={() => onDelete(connection.id)}>✕</button>
	</div>
</div>

<style>
	.row {
		display: grid;
		grid-template-columns: 1fr auto;
		gap: 4px;
		align-items: center;
		padding: 2px 8px;
	}
</style>
