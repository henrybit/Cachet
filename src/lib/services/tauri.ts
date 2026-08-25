import { invoke } from '@tauri-apps/api/core';
import { isTauriRuntime, toErrorMessage } from '$lib/services/runtime';

export async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
	if (!isTauriRuntime()) {
		throw new Error('请通过 npm run tauri 启动桌面应用后再执行该操作');
	}
	try {
		return await invoke<T>(command, args);
	} catch (error) {
		throw new Error(toErrorMessage(error));
	}
}
