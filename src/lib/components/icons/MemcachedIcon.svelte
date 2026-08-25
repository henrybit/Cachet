<script lang="ts">
	let { size = 18 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" aria-hidden="true">
	<rect x="5" y="5" width="14" height="3.2" rx="1.2" fill="white" opacity="0.95" />
	<rect x="5" y="10.4" width="14" height="3.2" rx="1.2" fill="white" opacity="0.75" />
	<rect x="5" y="15.8" width="14" height="3.2" rx="1.2" fill="white" opacity="0.55" />
</svg>
