<script>
	import router from "page";

	import Home from "./pages/Home.svelte";
	import About from "./pages/About.svelte";
	import Post from "./pages/Post.svelte";
	import Error404 from "./pages/Error404.svelte";

	let page;
	let params = {};

	router(
		"/",
		(ctx, next) => {
			params = ctx.params;
			next();
		},
		() => (page = Home)
	);
	router(
		"/about",
		(ctx, next) => {
			params = ctx.params;
			next();
		},
		() => (page = About)
	);
	router(
		"/post/:id",
		(ctx, next) => {
			params = ctx.params;
			next();
		},
		() => (page = Post)
	);
	router(
		"*",
		(ctx, next) => {
			params = ctx.params;
			next();
		},
		() => (page = Error404)
	);
	router.start();

	function isObjectEmpty(object) {
		return (
			Object.keys(object).length === 0 && object.constructor === Object
		);
	}
</script>

{#if isObjectEmpty(params)}
	<svelte:component this={page} />
{:else}
	<svelte:component this={page} {params} />
{/if}
