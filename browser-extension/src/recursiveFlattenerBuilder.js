let maxRecursionDebth = 100;

exports.createFlattener = (builderParams) => {

	let workerFunc = (item) =>  {

		let workingItems = [item];
		let scrapedContent = [];
		let index = 0;
		console.log("lenght is " + workingItems.length);

		while (workingItems.length > 0) {

			index = index + 1;
			if (index > maxRecursionDebth) {

				throw new Error("Infinite loop detected");
			}
			let currentItem = workingItems.pop();

			scrapedContent.push(builderParams.getNodeContent(currentItem));
			
			let nodeLinkedNodes = builderParams.getNodeLinkedNodes(currentItem);
			workingItems.concat(nodeLinkedNodes);

		}

		return scrapedContent;

	};
	return workerFunc;

}
