// IMPORTANT: Check domain to make sure script runs only on our window.
// https://github.com/tauri-apps/tauri/issues/4831
if (window.location.origin.includes("https://www.google.com")) {
	document.addEventListener("DOMContentLoaded", main);
}


async function main() {
	console.log(window);
	console.log(window.__TAURI__);
	document.body.style.backgroundColor = "#202030";
	setInterval(() => {
		console.log("ALIVE");
	}, 5000);
}

