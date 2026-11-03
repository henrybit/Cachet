import type { ConnectionConfig, ConnectionDraft } from '$lib/types/cache';
import { invokeCommand } from '$lib/services/tauri';

const BROWSER_KEY = 'cachet.connections';

function readBrowserConnections(): ConnectionConfig[] {
	if (typeof localStorage === 'undefined') return [];
	try {
		const raw = localStorage.getItem(BROWSER_KEY);
		return raw ? (JSON.parse(raw) as ConnectionConfig[]) : [];
	} catch {
		return [];
	}
}

function writeBrowserConnections(items: ConnectionConfig[]): void {
	localStorage.setItem(BROWSER_KEY, JSON.stringify(items));
}

export async function listConnections(): Promise<ConnectionConfig[]> {
	try {
		return await invokeCommand<ConnectionConfig[]>('list_connections');
	} catch {
		return readBrowserConnections();
	}
}

export async function upsertConnection(draft: ConnectionDraft): Promise<ConnectionConfig> {
	try {
		return await invokeCommand<ConnectionConfig>('upsert_connection', { draft });
	} catch (error) {
		if (typeof window === 'undefined' || '__TAURI_INTERNALS__' in window) throw error;
		const items = readBrowserConnections();
		const timestamp = Date.now();
		const existing = draft.id ? items.find((item) => item.id === draft.id) : undefined;
		const saved: ConnectionConfig = {
			id: existing?.id ?? crypto.randomUUID(),
			name: draft.name.trim(),
			engine: draft.engine,
			host: draft.host.trim(),
			port: draft.port,
			username: draft.username || null,
			hasPassword: Boolean(draft.password) || (existing?.hasPassword ?? false),
			database: draft.database ?? null,
			tls: draft.tls,
			timeoutMs: draft.timeoutMs,
			createdAt: existing?.createdAt ?? timestamp,
			updatedAt: timestamp
		};
		const next = existing
			? items.map((item) => (item.id === saved.id ? saved : item))
			: [...items, saved];
		writeBrowserConnections(next);
		return saved;
	}
}

export async function deleteConnection(id: string): Promise<void> {
	try {
		await invokeCommand('delete_connection', { id });
	} catch (error) {
		if (typeof window === 'undefined' || '__TAURI_INTERNALS__' in window) throw error;
		writeBrowserConnections(readBrowserConnections().filter((item) => item.id !== id));
	}
}

export async function testConnection(draft: ConnectionDraft): Promise<string> {
	return invokeCommand<string>('test_connection', { draft });
}
