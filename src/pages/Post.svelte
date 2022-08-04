<script>
	export let params = {};
	let world = "Post";
</script>

<h1>¡Hello {world}!</h1>
<h2>ID is {params.id}</h2>

<style>
	h1 {
		color: royalblue;
		text-transform: uppercase;
		font-size: 3em;
	}
	h2 {
		color: royalblue;
		font-size: 1.5em;
	}
</style>
