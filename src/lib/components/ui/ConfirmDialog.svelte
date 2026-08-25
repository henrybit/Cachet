<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';

	type Props = {
		open: boolean;
		title: string;
		message: string;
		confirmLabel?: string;
		onCancel: () => void;
		onConfirm: () => void;
	};

	let {
		open,
		title,
		message,
		confirmLabel = '删除',
		onCancel,
		onConfirm
	}: Props = $props();
</script>

<Modal {open} {title} onClose={onCancel}>
	<p class="muted">{message}</p>
	{#snippet footer()}
		<Button variant="ghost" onclick={onCancel}>取消</Button>
		<Button variant="danger" onclick={onConfirm}>{confirmLabel}</Button>
	{/snippet}
</Modal>
