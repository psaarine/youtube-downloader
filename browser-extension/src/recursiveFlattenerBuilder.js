let maxRecursionDebth = 100;

function getBookmarkNodeContent(node) {

	if (node.type == "bookmark") {

		return [node.url];
	} else {

		return [];
	}

}

function getNodeConnectedNodes(node) {

	if (node.type == "folder") {

		return node.children;
	} else {

		return [];
	};
}

function createFlattener(builderParams) {

	let workerFunc = (item) =>  {

		let workingItems = [item];
		let scrapedContent = [];
		let index = 0;

		while (workingItems.length > 0) {

			index = index + 1;
			if (index > maxRecursionDebth) {

				throw new Error("Infinite loop detected");
			}

			let currentItem = workingItems.pop();

			scrapedContent = scrapedContent.concat(builderParams.getNodeContent(currentItem));
			
			let nodeLinkedNodes = builderParams.getNodeLinkedNodes(currentItem);
			workingItems = workingItems.concat(nodeLinkedNodes);

		}

		return scrapedContent;

	};
	return workerFunc;
}

function createDefaultFlattener() {

	let builderParams = {};
	builderParams.getNodeContent = getBookmarkNodeContent;
	builderParams.getNodeLinkedNodes = getNodeConnectedNodes;

	return createFlattener(builderParams);
}
exports.createDefaultFlattener = createDefaultFlattener;
exports.createFlattener = createFlattener;
