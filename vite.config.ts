import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	clearScreen: false,
	server: {
		port: 5173,
		strictPort: true,
		host: '127.0.0.1'
	},
	envPrefix: ['VITE_', 'TAURI_'],
	plugins: [
		sveltekit({
			compilerOptions: {
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},
			adapter: adapter({
				fallback: 'index.html'
			})
		})
	]
});
