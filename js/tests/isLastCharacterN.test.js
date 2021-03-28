import isLastCharacterN from "../src/isLastCharacterN.js";

test.each([
  ["Aiden", true],
  ["Roxy", false],
  ["Bert", false],
  ["Dean", true],
  ["Ian", true],
  ["Brian", true],
  ["Daniel", false],
])("returns if last character is 'n'", (testInput, testOutput) => {
  let func = isLastCharacterN(testInput);
  expect(func).toBe(testOutput);
});
