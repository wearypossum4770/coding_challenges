import giveMeSomething from "../src/giveMeSomething.js";

test.each([
  ["a", "something a"],
  ["is cooking", "something is cooking"],
  [" is cooking", "something  is cooking"],
  ["is better than nothing", "something is better than nothing"],
  ["Bob Jane", "something Bob Jane"],
  ["something", "something something"],
])("function returns 'something' and %s", (testInput, testOutput) => {
  let func = giveMeSomething(testInput);
  expect(func).toStrictEqual(testOutput);
});
