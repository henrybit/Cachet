<script lang="ts">
	import EngineIcon from '$lib/components/icons/EngineIcon.svelte';
	import NumberField from '$lib/components/ui/NumberField.svelte';
	import TextField from '$lib/components/ui/TextField.svelte';
	import { ENGINE_LIST, ENGINE_META, createConnectionDraft, toDraft } from '$lib/config/engines';
	import type { CacheEngine, ConnectionDraft } from '$lib/types/cache';

	type Props = {
		initial: ConnectionDraft;
	};

	let { initial }: Props = $props();
	let draft = $state<ConnectionDraft>(createConnectionDraft());
	let redisDatabase = $state(0);

	$effect(() => {
		draft = toDraft(initial);
		redisDatabase = initial.database ?? 0;
	});

	function selectEngine(engine: CacheEngine): void {
		const currentDefault = ENGINE_META[draft.engine].defaultPort;
		draft.engine = engine;
		if (draft.port === currentDefault) {
			draft.port = ENGINE_META[engine].defaultPort;
		}
		draft.database = engine === 'redis' ? (draft.database ?? 0) : null;
		if (!draft.name.trim()) {
			draft = { ...createConnectionDraft(engine), ...draft, engine };
		}
	}

	export function getDraft(): ConnectionDraft {
		return {
			...draft,
			name: draft.name.trim(),
			host: draft.host.trim(),
			username: draft.username.trim(),
			password: draft.password,
			database: draft.engine === 'redis' ? redisDatabase : null
		};
	}
</script>

<div class="stack">
	<div class="engine-picker">
		{#each ENGINE_LIST as engine}
			<button
				class="engine-card {engine.id}"
				class:active={draft.engine === engine.id}
				type="button"
				onclick={() => selectEngine(engine.id)}
			>
				<EngineIcon engine={engine.id} size={36} />
				<div>
					<strong>{engine.label}</strong>
					<p class="hint">{engine.description}</p>
				</div>
			</button>
		{/each}
	</div>

	<TextField label="连接名称" placeholder="例如 本地 Redis" bind:value={draft.name} />
	<div class="grid-2">
		<TextField label="主机" placeholder="127.0.0.1" bind:value={draft.host} />
		<NumberField label="端口" min={1} max={65535} bind:value={draft.port} />
	</div>
	<div class="grid-2">
		<TextField label="用户名" placeholder="可选" bind:value={draft.username} />
		<TextField
			label="密码"
			type="password"
			placeholder={draft.id ? (draft.hasPassword ? '留空则保持原密码' : '可选') : '可选'}
			bind:value={draft.password}
		/>
	</div>
	<div class="grid-2">
		<NumberField
			label="超时 (ms)"
			min={500}
			max={30000}
			bind:value={draft.timeoutMs}
		/>
		{#if draft.engine === 'redis'}
			<NumberField label="数据库" min={0} max={15} bind:value={redisDatabase} />
		{:else}
			<div></div>
		{/if}
	</div>
	{#if draft.engine === 'redis'}
		<label class="check">
			<input type="checkbox" bind:checked={draft.tls} />
			启用 TLS（当前版本保存配置，连接时暂未启用）
		</label>
	{/if}
</div>

<style>
	.stack {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.check {
		display: flex;
		gap: 8px;
		align-items: center;
		color: var(--text-muted);
	}
</style>
