<script lang="ts">
	let { size = 16 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" aria-hidden="true">
	<rect x="4" y="6" width="16" height="12" rx="3" stroke="currentColor" stroke-width="1.6" />
	<circle cx="9" cy="12" r="1.4" fill="currentColor" />
	<path d="M11.2 12h6.3" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
</svg>
