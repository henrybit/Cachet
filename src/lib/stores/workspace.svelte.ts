import * as cacheService from '$lib/services/cache';
import { toErrorMessage } from '$lib/services/runtime';
import type { CacheEntry, InfoItem, KeySummary } from '$lib/types/cache';

class WorkspaceStore {
	pattern = $state('*');
	lookupKey = $state('');
	database = $state(0);
	keys = $state<KeySummary[]>([]);
	cursor = $state<string | null>(null);
	done = $state(true);
	selectedKey = $state<string | null>(null);
	entry = $state<CacheEntry | null>(null);
	info = $state<InfoItem[]>([]);
	loadingKeys = $state(false);
	loadingEntry = $state(false);
	saving = $state(false);
	error = $state<string | null>(null);
	notice = $state<string | null>(null);

	reset(): void {
		this.pattern = '*';
		this.lookupKey = '';
		this.database = 0;
		this.keys = [];
		this.cursor = null;
		this.done = true;
		this.selectedKey = null;
		this.entry = null;
		this.info = [];
		this.error = null;
		this.notice = null;
	}

	async open(connectionId: string, database = 0): Promise<void> {
		this.reset();
		this.database = database;
		await Promise.all([this.refreshInfo(connectionId), this.refreshKeys(connectionId, true)]);
	}

	async refreshInfo(connectionId: string): Promise<void> {
		try {
			this.info = await cacheService.fetchServerInfo(connectionId, this.database);
		} catch (error) {
			this.info = [];
			this.error = toErrorMessage(error, '读取服务器信息失败');
		}
	}

	async refreshKeys(connectionId: string, restart = true): Promise<void> {
		this.loadingKeys = true;
		this.error = null;
		try {
			const page = await cacheService.scanKeys(
				connectionId,
				this.pattern.trim() || '*',
				restart ? null : this.cursor,
				this.database
			);
			this.keys = restart ? page.keys : [...this.keys, ...page.keys];
			this.cursor = page.cursor ?? null;
			this.done = page.done;
		} catch (error) {
			if (restart) this.keys = [];
			this.error = toErrorMessage(error, '扫描键失败');
		} finally {
			this.loadingKeys = false;
		}
	}

	async selectKey(connectionId: string, key: string): Promise<void> {
		this.selectedKey = key;
		this.loadingEntry = true;
		this.error = null;
		try {
			this.entry = await cacheService.getEntry(connectionId, key, this.database);
			this.lookupKey = key;
		} catch (error) {
			this.entry = null;
			this.error = toErrorMessage(error, '读取键失败');
		} finally {
			this.loadingEntry = false;
		}
	}

	async lookup(connectionId: string): Promise<void> {
		const key = this.lookupKey.trim();
		if (!key) return;
		if (!this.keys.some((item) => item.key === key)) {
			this.keys = [{ key, keyType: 'string', ttl: null }, ...this.keys];
		}
		await this.selectKey(connectionId, key);
	}

	startCreate(): void {
		this.selectedKey = null;
		this.entry = {
			key: '',
			value: '',
			keyType: 'string',
			ttl: -1,
			size: 0
		};
		this.notice = null;
		this.error = null;
	}

	async saveEntry(connectionId: string, key: string, value: string, ttlSeconds?: number | null): Promise<void> {
		this.saving = true;
		this.error = null;
		this.notice = null;
		try {
			await cacheService.setEntry(connectionId, key, value, ttlSeconds, this.database);
			this.notice = '已保存';
			if (!this.keys.some((item) => item.key === key)) {
				this.keys = [{ key, keyType: 'string', ttl: ttlSeconds ?? -1 }, ...this.keys];
			}
			await this.selectKey(connectionId, key);
		} catch (error) {
			this.error = toErrorMessage(error, '保存键失败');
		} finally {
			this.saving = false;
		}
	}

	async removeEntry(connectionId: string, key: string): Promise<void> {
		this.error = null;
		try {
			await cacheService.deleteEntry(connectionId, key, this.database);
			this.keys = this.keys.filter((item) => item.key !== key);
			if (this.selectedKey === key) {
				this.selectedKey = null;
				this.entry = null;
			}
			this.notice = '键已删除';
		} catch (error) {
			this.error = toErrorMessage(error, '删除键失败');
		}
	}
}

export const workspace = new WorkspaceStore();
