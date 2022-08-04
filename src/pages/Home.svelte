<script>
	import { invoke } from "@tauri-apps/api";
	import { WebviewWindow } from "@tauri-apps/api/window";
	let count = 0;
	let input = "World";
	let message = "·";
	let width = 1280;
	let height = 720;
	let webview;

	function doInvoke() {
		invoke("greet", { name: input }).then((response) => {
			message = response;
		});
	}

	function doOpenGoogleT() {
		invoke("open_google").then((result) => {
			console.log("Result:", result);
			console.log(WebviewWindow.getByLabel("exGoogle"));
		});
	}

	function doMinimizeT() {
		if (WebviewWindow.getByLabel("exGoogle")) {
			console.log(WebviewWindow.getByLabel("exGoogle"));
			WebviewWindow.getByLabel("exGoogle").minimize();
		}
	}

	function doOpenGoogleL() {
		webview = new WebviewWindow("wwGoogle", {
			title: "Google",
			width: width,
			height: height,
			url: "https://www.google.com/"
		});

		console.log(webview);

		webview.once("tauri://created", function (e) {
			console.log(e);
		});
		webview.once("tauri://error", function (e) {
			console.log(e);
		});
	}
	function doMinimizeL() {
		if (webview) {
			webview.minimize();
		}
	}
</script>

<div>
	<button on:click={() => count--}>&minus;</button>
	<span>{count}</span>
	<button on:click={() => count++}>&plus;</button>
</div>
<hr />
<div id="invoke">
	<input type="text" bind:value={input} />
	<button on:click={doInvoke}>Send to Tauri</button>
	<p>{message}</p>
</div>
<hr />
<div id="external">
	<div>
		<button on:click={doOpenGoogleT}>OPEN Google from Tauri</button>
		<button on:click={doMinimizeT}>Minimize</button>
	</div>
	<p>External Script will change page background</p>
</div>
<hr />
<div id="local">
	<div>
		<input id="width" type="number" bind:value={width} />
		<input type="number" bind:value={height} />
	</div>
	<div>
		<button on:click={doOpenGoogleL}>OPEN Google Locally</button>
		<button on:click={doMinimizeL}>Minimize</button>
	</div>
	<p>Can't inject scripts using this method</p>
</div>
<hr />
<div>
	<p>Regular link: <a href="https://google.com">GOOGLE</a></p>
</div>

<style>
	div {
		color: aliceblue;
		font-size: 2em;
		width: 80vw;
		margin: 0px auto;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 12px;
		padding: 12px;
	}
	span {
		width: 12vw;
	}
	input {
		font-size: 1.5rem;
	}
	button {
		padding: 0.25rem 0.75rem;
		font-size: 1.5rem;
		cursor: pointer;
	}
	#invoke {
		display: flex;
		flex-direction: column;
	}
	p {
		font-size: 1rem;
	}
	#external,
	#local {
		display: flex;
		flex-direction: column;
	}
	a {
		color: dodgerblue;
	}
</style>
