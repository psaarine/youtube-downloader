import getBookmarks from "./getBookmarks"
import tryConnect from "./connection"

let pollIntervalMs = 5000;
let bookmarks = undefined;
let pollFunction = undefined;
let conn = undefined;

function setStatusToConnected(){

	clearInterval(pollFunction);
	pollFunction = setInterval(getAndSendBookmarks, pollIntervalMs);
	
}

function setStatusToConnecting() {

	clearInterval(pollFunction);
	pollFunction = setInterval(tryConnect, pollIntervalMs);
	
}

function getAndSendBookmarks() {

	let bookmarks = getBookmarks(logBookmarks);
}

 function logBookmarks(bookmarks) {

	 console.log(bookmarks);
 }

function startApplication() {

	pollFunction = setInterval(getAndSendBookmarks, 5000);
};

startApplication();
