<script lang="ts">
	import KeyEditor from '$lib/components/cache/KeyEditor.svelte';
	import KeyList from '$lib/components/cache/KeyList.svelte';
	import ServerInfo from '$lib/components/cache/ServerInfo.svelte';
	import EngineIcon from '$lib/components/icons/EngineIcon.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import StatusText from '$lib/components/ui/StatusText.svelte';
	import { ENGINE_META, formatEndpoint } from '$lib/config/engines';
	import { workspace } from '$lib/stores/workspace.svelte';
	import type { ConnectionConfig } from '$lib/types/cache';

	let { connection }: { connection: ConnectionConfig | null } = $props();
	const meta = $derived(connection ? ENGINE_META[connection.engine] : null);
</script>

{#if !connection || !meta}
	<EmptyState title="选择一个缓存连接" description="在左侧保存 Redis 或 Memcached 连接后，即可浏览和编辑缓存数据。" />
{:else}
	<section class="workspace">
		<div class="panel">
			<div class="section-head">
				<div class="identity">
					<EngineIcon engine={connection.engine} size={32} />
					<div>
						<h2 class="section-title">{connection.name}</h2>
						<p class="hint">{meta.label} · {formatEndpoint(connection.host, connection.port)}</p>
					</div>
				</div>
				<Button onclick={() => workspace.startCreate()}>新建键</Button>
			</div>
			<div class="toolbar">
				<input class="control" placeholder="SCAN 模式，例如 user:*" bind:value={workspace.pattern} />
				<Button loading={workspace.loadingKeys} onclick={() => workspace.refreshKeys(connection.id, true)}>
					扫描
				</Button>
			</div>
			{#if connection.engine === 'redis'}
				<div class="toolbar">
					<label class="hint" for="db">DB</label>
					<input
						id="db"
						class="control"
						type="number"
						min="0"
						max="15"
						bind:value={workspace.database}
						onchange={() => workspace.refreshKeys(connection.id, true)}
					/>
				</div>
			{/if}
			<div class="toolbar">
				<input class="control" placeholder="按精确键名读取" bind:value={workspace.lookupKey} />
				<Button onclick={() => workspace.lookup(connection.id)}>读取</Button>
			</div>
			<StatusText text={workspace.error} tone="danger" />
			{#if workspace.keys.length === 0 && !workspace.loadingKeys}
				<EmptyState title="没有扫描到键" description="可以调整匹配模式，或直接输入精确键名读取。" />
			{:else}
				<KeyList
					keys={workspace.keys}
					selectedKey={workspace.selectedKey}
					onSelect={(key) => workspace.selectKey(connection.id, key)}
				/>
			{/if}
			{#if !workspace.done}
				<div class="toolbar">
					<Button loading={workspace.loadingKeys} onclick={() => workspace.refreshKeys(connection.id, false)}>
						加载更多
					</Button>
				</div>
			{/if}
		</div>
		<div class="editor">
			<div class="section-head">
				<h2 class="section-title">键值编辑</h2>
				<Button variant="ghost" onclick={() => workspace.refreshInfo(connection.id)}>刷新信息</Button>
			</div>
			<StatusText text={workspace.notice} tone="success" />
			<KeyEditor
				engine={connection.engine}
				entry={workspace.entry}
				saving={workspace.saving}
				onSave={(key, value, ttl) => workspace.saveEntry(connection.id, key, value, ttl)}
				onDelete={(key) => workspace.removeEntry(connection.id, key)}
			/>
			<ServerInfo items={workspace.info} />
		</div>
	</section>
{/if}

<style>
	.identity {
		display: flex;
		align-items: center;
		gap: 10px;
	}
</style>
