<script lang="ts">
	import KeyIcon from '$lib/components/icons/KeyIcon.svelte';
	import { formatTtl } from '$lib/config/engines';
	import type { KeySummary } from '$lib/types/cache';

	type Props = {
		keys: KeySummary[];
		selectedKey: string | null;
		onSelect: (key: string) => void;
	};

	let { keys, selectedKey, onSelect }: Props = $props();
</script>

<div class="scroll">
	{#each keys as item (item.key)}
		<button
			class="list-item"
			class:active={item.key === selectedKey}
			type="button"
			onclick={() => onSelect(item.key)}
		>
			<span class="key-mark"><KeyIcon size={15} /></span>
			<div class="item-copy">
				<strong>{item.key}</strong>
				<span>{item.keyType ?? 'string'} · {formatTtl(item.ttl)}</span>
			</div>
		</button>
	{/each}
</div>

<style>
	.key-mark {
		display: grid;
		place-items: center;
		width: 24px;
		height: 24px;
		color: var(--text-muted);
	}
</style>
