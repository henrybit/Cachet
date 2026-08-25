<script lang="ts">
	import Field from '$lib/components/ui/Field.svelte';

	type Props = {
		label: string;
		value?: string;
		type?: string;
		placeholder?: string;
		disabled?: boolean;
		hint?: string;
	};

	let {
		label,
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		disabled = false,
		hint
	}: Props = $props();
</script>

<Field {label} {hint}>
	<input class="control" {type} {placeholder} {disabled} autocomplete="off" bind:value />
</Field>
