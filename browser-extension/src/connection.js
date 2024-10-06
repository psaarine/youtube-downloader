function onConnectionError(err) {

	console.log("Error connecting to application: " + err);
}

export default function tryConnect(onConnection, onConnectionDropped){

	let conn = WebSocket("http://127.0.0.1:1234");
	conn.onopen = onConnection;
	conn.onclose = onConnectionDropped;
}
