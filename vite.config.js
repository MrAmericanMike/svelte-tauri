import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
	root: "src",
	plugins: [svelte()],
	server: {
		port: 3000
	},
	preview: {
		port: 8080
	},
	build: {
		target: "es2015",
		minify: false,
		assetsInlineLimit: 0,
		outDir: "../public",
		emptyOutDir: true,
		rollupOptions: {
			output: {
				entryFileNames: "assets/[name].js",
				chunkFileNames: "assets/[name].js",
				assetFileNames: "assets/[name].[ext]"
			}
		}
	}
});

