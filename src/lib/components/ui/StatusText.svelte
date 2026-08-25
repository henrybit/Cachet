<script lang="ts">
	let { text = '', tone = 'muted' }: { text?: string | null; tone?: 'muted' | 'success' | 'danger' } = $props();
</script>

{#if text}
	<p class={tone === 'danger' ? 'error-banner' : tone === 'success' ? 'notice' : 'hint'}>{text}</p>
{/if}
