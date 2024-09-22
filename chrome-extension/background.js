function set_state_to_connected(conn) {

	clearInterval(tryConnectIntervalId);
	console.log("Opened connection");
	activeConnection = conn;
	sendDataIntervalId = setInterval(send_data, 5000);
}
function attempt_connect() {

	console.log("Attempted to connect");
	const socket = new WebSocket("http://127.0.0.1:1234");

	socket.onopen = (event) => {
		set_state_to_connected(socket);
	};
	socket.onerror = (event) => {
		console.log(event);
	};

	socket.onmessage = (event) => {

		console.log(event);l
	};

	socket.onclose = (event) => {

		console.log(event);
	};
}

function send_data() {

	activeConnection.send("Lol");

}
let tryConnectIntervalId = setInterval(attempt_connect, 5000);
let activeConnection = null;
let sendDataIntervalId = null;
