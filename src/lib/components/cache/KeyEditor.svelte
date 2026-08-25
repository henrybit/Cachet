<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import NumberField from '$lib/components/ui/NumberField.svelte';
	import TextField from '$lib/components/ui/TextField.svelte';
	import type { CacheEngine, CacheEntry } from '$lib/types/cache';

	type Props = {
		engine: CacheEngine;
		entry: CacheEntry | null;
		saving?: boolean;
		onSave: (key: string, value: string, ttlSeconds?: number | null) => void;
		onDelete: (key: string) => void;
	};

	let { engine, entry, saving = false, onSave, onDelete }: Props = $props();

	let key = $state('');
	let value = $state('');
	let ttl = $state(0);

	$effect(() => {
		key = entry?.key ?? '';
		value = entry?.value ?? '';
		ttl = entry && entry.ttl > 0 ? entry.ttl : 0;
	});

	const editable = $derived(!entry || entry.keyType === 'string' || entry.key === '');
</script>

{#if !entry}
	<div class="empty">
		<strong>选择一个键</strong>
		<p class="muted">从左侧列表点选，或按精确键名读取 / 新建。</p>
	</div>
{:else}
	<div class="editor-form">
		<div class="grid-2">
			<TextField label="键名" bind:value={key} />
			<NumberField
				label="TTL（秒，空或 0 表示永久）"
				min={0}
				bind:value={ttl}
			/>
		</div>
		<div class="meta muted">
			类型 {entry.keyType} · 大小 {entry.size} 字节
			{#if engine === 'memcached'}
				· Memcached 以字符串存储
			{/if}
		</div>
		<label class="field">
			<span>值</span>
			<textarea class="textarea" bind:value disabled={!editable}></textarea>
		</label>
		<div class="actions">
			<Button
				variant="primary"
				loading={saving}
				disabled={!editable || !key.trim()}
				onclick={() => onSave(key.trim(), value, ttl > 0 ? ttl : null)}
			>
				保存
			</Button>
			{#if entry.key}
				<Button variant="danger" disabled={saving} onclick={() => onDelete(entry.key)}>删除键</Button>
			{/if}
		</div>
	</div>
{/if}

<style>
	.editor-form {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding: 0 16px 16px;
		min-height: 0;
	}

	.meta {
		font-size: 12px;
	}
</style>
