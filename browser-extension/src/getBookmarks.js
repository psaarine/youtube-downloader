function isBookmarkNode(node) {

	return node.type == 'bookmark' && node.url != undefined;
}

function mapBookmarkToUrl(bookmark){

	return bookmark.url;

}
function mapTreeToUrls(tree) {

	return tree.every(isBookmarkNode)
	.map(mapBookmarkToUrl);

}

function filterDownloadedBookmarks(tree) {

	if (tree.title == 'music') {

		return mapTreeToUrls(tree);
	} else {

		return tree.children.flatMap(filterDownloadedBookmarks);
	}

}

export default function getBookmars(bookmarkHandler) {

	let bookmarks = browser.bookmarks.getTree().then((result) => {bookmarkHandler(result)});
}
