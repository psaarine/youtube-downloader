
const builder = require('../src/recursiveFlattenerBuilder');

function getNodeContent(node) {

	if (node.type == "url") {

		return [node.url];
	} else {

		return [];
	}

}

function getNodeConnectedNodes(node) {

	if (node.type == "bookmark") {

		return node.children;
	} else {

		return [];
	};
}
function createDummyBookmark(title) {

	return { title: title, type: "bookmark" };
}

function createDummyItem(url) {

	return { type: "url", url: url }
}
function createDefaultMockFlattener() {

	let builderParams = {};
	builderParams.getNodeContent = getNodeContent;
	builderParams.getNodeLinkedNodes = getNodeConnectedNodes;

	return builder.createFlattener(builderParams);
}
test('can process singular matching item', () => {


	let item = createDummyBookmark("music", "bookmark");
	let firstChild = createDummyItem("lol.com");
	let secondChild = createDummyItem("rekt.com");
	let thirdChild = createDummyItem("sec.com");

	item.children = { firstChild, secondChild, thirdChild };
	let flattener = createDefaultMockFlattener();

	let flattened = flattener(item);

	//expect(flattened.length).toBe(3);
	
})
