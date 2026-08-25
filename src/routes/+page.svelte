<script lang="ts">
	import { onMount } from 'svelte';
	import CacheWorkspace from '$lib/components/cache/CacheWorkspace.svelte';
	import ConnectionSidebar from '$lib/components/connection/ConnectionSidebar.svelte';
	import { connections } from '$lib/stores/connections.svelte';
	import { workspace } from '$lib/stores/workspace.svelte';

	onMount(() => {
		void connections.load();
	});

	async function selectConnection(id: string): Promise<void> {
		connections.select(id);
		const selected = connections.selected;
		if (!selected) return;
		await workspace.open(selected.id, selected.database ?? 0);
	}
</script>

<div class="app-shell">
	<ConnectionSidebar onSelect={selectConnection} />
	<CacheWorkspace connection={connections.selected} />
</div>
