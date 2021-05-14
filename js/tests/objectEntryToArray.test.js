import objectEntryToArray from "../src/objectEntryToArray.js";
test.each([
  [
    { D: 1, B: 2, C: 3 },
    [
      ["D", 1],
      ["B", 2],
      ["C", 3],
    ],
  ],
  [
    { likes: 2, dislikes: 3, followers: 10 },
    [
      ["likes", 2],
      ["dislikes", 3],
      ["followers", 10],
    ],
  ],
])("Convert object to array", (obj, output) => {
  let func = objectEntryToArray(obj);
  expect(func).toStrictEqual(output);
});
