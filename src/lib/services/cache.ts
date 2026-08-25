import type { CacheEntry, InfoItem, KeyPage } from '$lib/types/cache';
import { invokeCommand } from '$lib/services/tauri';

export async function fetchServerInfo(id: string, database?: number | null): Promise<InfoItem[]> {
	return invokeCommand<InfoItem[]>('fetch_server_info', { id, database: database ?? null });
}

export async function scanKeys(
	id: string,
	pattern: string,
	cursor?: string | null,
	database?: number | null
): Promise<KeyPage> {
	return invokeCommand<KeyPage>('scan_keys', {
		id,
		pattern,
		cursor: cursor ?? null,
		database: database ?? null
	});
}

export async function getEntry(id: string, key: string, database?: number | null): Promise<CacheEntry> {
	return invokeCommand<CacheEntry>('get_entry', { id, key, database: database ?? null });
}

export async function setEntry(
	id: string,
	key: string,
	value: string,
	ttlSeconds?: number | null,
	database?: number | null
): Promise<void> {
	await invokeCommand('set_entry', {
		id,
		key,
		value,
		ttlSeconds: ttlSeconds ?? null,
		database: database ?? null
	});
}

export async function deleteEntry(id: string, key: string, database?: number | null): Promise<boolean> {
	return invokeCommand<boolean>('delete_entry', { id, key, database: database ?? null });
}
