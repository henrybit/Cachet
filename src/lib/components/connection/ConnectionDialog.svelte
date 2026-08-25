<script lang="ts">
	import ConnectionForm from '$lib/components/connection/ConnectionForm.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import StatusText from '$lib/components/ui/StatusText.svelte';
	import type { ConnectionDraft } from '$lib/types/cache';

	type Props = {
		open: boolean;
		title: string;
		initial: ConnectionDraft | null;
		saving?: boolean;
		testing?: boolean;
		error?: string | null;
		notice?: string | null;
		onClose: () => void;
		onSave: (draft: ConnectionDraft) => void;
		onTest: (draft: ConnectionDraft) => void;
	};

	let {
		open,
		title,
		initial,
		saving = false,
		testing = false,
		error,
		notice,
		onClose,
		onSave,
		onTest
	}: Props = $props();

	let form = $state<ConnectionForm | null>(null);

	function submit(action: 'save' | 'test'): void {
		const draft = form?.getDraft();
		if (!draft) return;
		if (action === 'save') onSave(draft);
		else onTest(draft);
	}
</script>

<Modal {open} {title} {onClose}>
	{#if initial}
		<ConnectionForm bind:this={form} {initial} />
		<StatusText text={notice} tone="success" />
		<StatusText text={error} tone="danger" />
	{/if}
	{#snippet footer()}
		<Button variant="ghost" onclick={onClose}>取消</Button>
		<div class="actions">
			<Button loading={testing} disabled={saving} onclick={() => submit('test')}>测试连接</Button>
			<Button variant="primary" loading={saving} disabled={testing} onclick={() => submit('save')}>保存</Button>
		</div>
	{/snippet}
</Modal>
