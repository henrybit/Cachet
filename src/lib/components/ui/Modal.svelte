<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		open: boolean;
		title: string;
		onClose: () => void;
		children: Snippet;
		footer?: Snippet;
	};

	let { open, title, onClose, children, footer }: Props = $props();
</script>

{#if open}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={onClose}
		onkeydown={(event) => event.key === 'Escape' && onClose()}
	>
		<div
			class="modal"
			role="dialog"
			tabindex="-1"
			aria-modal="true"
			aria-labelledby="modal-title"
			onclick={(event) => event.stopPropagation()}
			onkeydown={(event) => event.stopPropagation()}
		>
			<div class="modal-head">
				<h2 id="modal-title" class="section-title">{title}</h2>
				<button class="btn ghost icon" type="button" onclick={onClose} aria-label="关闭">×</button>
			</div>
			<div class="modal-body">
				{@render children()}
			</div>
			{#if footer}
				<div class="modal-foot">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}
