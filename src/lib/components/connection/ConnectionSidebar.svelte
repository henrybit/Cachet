<script lang="ts">
	import ConnectionDialog from '$lib/components/connection/ConnectionDialog.svelte';
	import ConnectionList from '$lib/components/connection/ConnectionList.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import { connections } from '$lib/stores/connections.svelte';

	let query = $state('');

	let { onSelect }: { onSelect: (id: string) => void } = $props();
</script>

<aside class="sidebar">
	<div class="section-head">
		<div>
			<p class="hint" style="margin: 0 0 4px;">缓存数据管理</p>
			<h1 class="section-title">Cachet</h1>
		</div>
		<Button variant="primary" onclick={() => connections.openCreate()}>新建</Button>
	</div>
	<div class="toolbar">
		<input class="control" placeholder="搜索连接" bind:value={query} />
	</div>
	<ConnectionList
		items={connections.items}
		selectedId={connections.selectedId}
		{query}
		onSelect={onSelect}
		onEdit={(item) => connections.openEdit(item)}
		onDelete={(id) => connections.askDelete(id)}
	/>
</aside>

<ConnectionDialog
	open={connections.dialogOpen}
	title={connections.editing?.id ? '编辑连接' : '新建连接'}
	initial={connections.editing}
	saving={connections.saving}
	testing={connections.testing}
	error={connections.error}
	notice={connections.notice}
	onClose={() => connections.closeDialog()}
	onSave={(draft) => connections.save(draft)}
	onTest={(draft) => connections.test(draft)}
/>

<ConfirmDialog
	open={Boolean(connections.pendingDelete)}
	title="删除连接"
	message={`确定删除「${connections.pendingDelete?.name ?? ''}」吗？此操作不会清空缓存服务器中的数据。`}
	onCancel={() => connections.cancelDelete()}
	onConfirm={() => connections.pendingDeleteId && connections.remove(connections.pendingDeleteId)}
/>
