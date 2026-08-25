<script lang="ts">
	import Field from '$lib/components/ui/Field.svelte';

	type Props = {
		label: string;
		value?: number | null;
		min?: number;
		max?: number;
		hint?: string;
		disabled?: boolean;
	};

	let {
		label,
		value = $bindable(0),
		min = 0,
		max,
		hint,
		disabled = false
	}: Props = $props();
</script>

<Field {label} {hint}>
	<input class="control" type="number" {min} {max} {disabled} bind:value />
</Field>
