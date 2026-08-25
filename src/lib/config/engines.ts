import type { CacheEngine, ConnectionConfig, ConnectionDraft } from '$lib/types/cache';

export type EngineMeta = {
	id: CacheEngine;
	label: string;
	shortLabel: string;
	defaultPort: number;
	description: string;
	accent: string;
};

export const ENGINE_META: Record<CacheEngine, EngineMeta> = {
	redis: {
		id: 'redis',
		label: 'Redis',
		shortLabel: 'Redis',
		defaultPort: 6379,
		description: '内存数据结构存储，支持 TTL、多数据库和丰富的键类型。',
		accent: '#e11d48'
	},
	memcached: {
		id: 'memcached',
		label: 'Memcached',
		shortLabel: 'Memcached',
		defaultPort: 11211,
		description: '高性能分布式内存对象缓存，适合简单 KV 场景。',
		accent: '#14b8a6'
	}
};

export const ENGINE_LIST = Object.values(ENGINE_META);

export function createConnectionDraft(engine: CacheEngine = 'redis'): ConnectionDraft {
	const meta = ENGINE_META[engine];
	return {
		id: null,
		name: '',
		engine,
		host: '127.0.0.1',
		port: meta.defaultPort,
		username: '',
		password: '',
		database: engine === 'redis' ? 0 : null,
		tls: false,
		timeoutMs: 3000
	};
}

export function toDraft(source: ConnectionConfig | ConnectionDraft): ConnectionDraft {
	return {
		id: source.id ?? null,
		name: source.name,
		engine: source.engine,
		host: source.host,
		port: source.port,
		username: source.username ?? '',
		password: 'password' in source ? (source.password ?? '') : '',
		hasPassword: 'hasPassword' in source ? source.hasPassword : false,
		database: source.engine === 'redis' ? (source.database ?? 0) : null,
		tls: source.tls,
		timeoutMs: source.timeoutMs
	};
}

export function formatEndpoint(host: string, port: number): string {
	return `${host}:${port}`;
}

export function formatTtl(ttl?: number | null): string {
	if (ttl == null || ttl < 0) return '永久';
	if (ttl === 0) return '即将过期';
	if (ttl < 60) return `${ttl}s`;
	if (ttl < 3600) return `${Math.floor(ttl / 60)}m`;
	if (ttl < 86400) return `${Math.floor(ttl / 3600)}h`;
	return `${Math.floor(ttl / 86400)}d`;
}
