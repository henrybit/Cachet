export function isTauriRuntime(): boolean {
	return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

export function toErrorMessage(error: unknown, fallback = '操作失败'): string {
	if (typeof error === 'string' && error.trim()) return error;
	if (error instanceof Error && error.message) return error.message;
	return fallback;
}
