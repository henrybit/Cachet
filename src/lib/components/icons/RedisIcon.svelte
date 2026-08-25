<script lang="ts">
	let { size = 18 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 24 24" fill="none" aria-hidden="true">
	<path
		d="M5 8.5c0-1.4 3.1-2.5 7-2.5s7 1.1 7 2.5v2c0 1.4-3.1 2.5-7 2.5s-7-1.1-7-2.5v-2Z"
		fill="white"
		opacity="0.95"
	/>
	<path
		d="M5 12.5v2c0 1.4 3.1 2.5 7 2.5s7-1.1 7-2.5v-2c-1.4 1.1-4 1.8-7 1.8s-5.6-.7-7-1.8Z"
		fill="white"
		opacity="0.8"
	/>
	<path
		d="M5 16.2v1.3c0 1.4 3.1 2.5 7 2.5s7-1.1 7-2.5v-1.3c-1.4 1.1-4 1.8-7 1.8s-5.6-.7-7-1.8Z"
		fill="white"
		opacity="0.62"
	/>
</svg>
