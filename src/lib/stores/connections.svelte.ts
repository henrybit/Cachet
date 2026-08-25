import { createConnectionDraft, toDraft } from '$lib/config/engines';
import * as connectionService from '$lib/services/connection';
import { toErrorMessage } from '$lib/services/runtime';
import type { ConnectionConfig, ConnectionDraft } from '$lib/types/cache';

class ConnectionStore {
	items = $state<ConnectionConfig[]>([]);
	selectedId = $state<string | null>(null);
	loading = $state(false);
	saving = $state(false);
	testing = $state(false);
	error = $state<string | null>(null);
	notice = $state<string | null>(null);
	dialogOpen = $state(false);
	editing = $state<ConnectionDraft | null>(null);
	pendingDeleteId = $state<string | null>(null);

	get selected(): ConnectionConfig | null {
		return this.items.find((item) => item.id === this.selectedId) ?? null;
	}

	get pendingDelete(): ConnectionConfig | null {
		return this.items.find((item) => item.id === this.pendingDeleteId) ?? null;
	}

	async load(): Promise<void> {
		this.loading = true;
		this.error = null;
		try {
			this.items = await connectionService.listConnections();
			if (this.selectedId && !this.items.some((item) => item.id === this.selectedId)) {
				this.selectedId = null;
			}
		} catch (error) {
			this.error = toErrorMessage(error, '加载连接失败');
		} finally {
			this.loading = false;
		}
	}

	select(id: string | null): void {
		this.selectedId = id;
	}

	openCreate(): void {
		this.editing = createConnectionDraft('redis');
		this.dialogOpen = true;
		this.notice = null;
		this.error = null;
	}

	openEdit(connection: ConnectionConfig): void {
		this.editing = { ...toDraft(connection), password: '', hasPassword: connection.hasPassword };
		this.dialogOpen = true;
		this.notice = null;
		this.error = null;
	}

	closeDialog(): void {
		this.dialogOpen = false;
		this.editing = null;
		this.testing = false;
	}

	askDelete(id: string): void {
		this.pendingDeleteId = id;
	}

	cancelDelete(): void {
		this.pendingDeleteId = null;
	}

	async save(draft: ConnectionDraft): Promise<void> {
		this.saving = true;
		this.error = null;
		try {
			const saved = await connectionService.upsertConnection(draft);
			await this.load();
			this.selectedId = saved.id;
			this.closeDialog();
		} catch (error) {
			this.error = toErrorMessage(error, '保存连接失败');
		} finally {
			this.saving = false;
		}
	}

	async test(draft: ConnectionDraft): Promise<void> {
		this.testing = true;
		this.notice = null;
		this.error = null;
		try {
			this.notice = await connectionService.testConnection(draft);
		} catch (error) {
			this.error = toErrorMessage(error, '测试连接失败');
		} finally {
			this.testing = false;
		}
	}

	async remove(id: string): Promise<void> {
		this.error = null;
		try {
			await connectionService.deleteConnection(id);
			if (this.selectedId === id) this.selectedId = null;
			this.pendingDeleteId = null;
			await this.load();
		} catch (error) {
			this.error = toErrorMessage(error, '删除连接失败');
		}
	}
}

export const connections = new ConnectionStore();
