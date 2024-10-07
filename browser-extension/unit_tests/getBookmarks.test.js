const builder = require('../src/recursiveFlattenerBuilder');

function createDummyFolder(title) {

	return { title: title, type: "folder" };
}

function createDummyBookmark(url) {

	return { type: "bookmark", url: url }
}
test('can process singular matching item', () => {


	let item = createDummyFolder("music");
	let firstChild = createDummyBookmark("lol.com");
	let secondChild = createDummyBookmark("rekt.com");
	let thirdChild = createDummyBookmark("sec.com");

	item.children = [firstChild, secondChild, thirdChild];
	let flattener = builder.createDefaultFlattener();

	let flattened = flattener(item);

	expect(flattened.length).toBe(3);
	
})
