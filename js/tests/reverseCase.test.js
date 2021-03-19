import reverseCase from "../src/reverseCase.js";

test.each([
  ["Happy Birthday", "hAPPY bIRTHDAY"],
  ["MANY THANKS", "many thanks"],
  ["sPoNtAnEoUs", "SpOnTaNeOuS"],
  ["eXCELLENT, yOuR mAJESTY", "Excellent, YoUr Majesty"],
])("%s character's casing reversed to %s", (testInput, testOutput) => {
  let func = reverseCase(testInput);
  expect(func).toBe(testOutput);
});
