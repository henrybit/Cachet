export type CacheEngine = 'redis' | 'memcached';

export type ConnectionConfig = {
	id: string;
	name: string;
	engine: CacheEngine;
	host: string;
	port: number;
	username?: string | null;
	hasPassword: boolean;
	database?: number | null;
	tls: boolean;
	timeoutMs: number;
	createdAt: number;
	updatedAt: number;
};

export type ConnectionDraft = {
	id?: string | null;
	name: string;
	engine: CacheEngine;
	host: string;
	port: number;
	username: string;
	password: string;
	hasPassword?: boolean;
	database: number | null;
	tls: boolean;
	timeoutMs: number;
};

export type KeySummary = {
	key: string;
	keyType?: string | null;
	ttl?: number | null;
};

export type KeyPage = {
	keys: KeySummary[];
	cursor?: string | null;
	done: boolean;
	supportsScan: boolean;
};

export type CacheEntry = {
	key: string;
	value: string;
	keyType: string;
	ttl: number;
	size: number;
};

export type InfoItem = {
	key: string;
	value: string;
};
