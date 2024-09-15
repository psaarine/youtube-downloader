let port = browser.runtime.connectNative("a");
console.log(port);
port.onDisconnect.addListener((p) => {

	if (p.error) {

		console.log("Disconnected due to error: " + p.error.message);
	}
});
if (port.error) {

	console.log("Failed to initialize runtime port: " + port.error);
} else {

	console.log("Initialized runtime port");
}
port.onMessage.addListener((resp) => {

	console.log("Received message");
});

function myFunction() {

	port.postMessage("Lol");
}
window.setTimeout(myFunction, 3000);
