import getOddNumbers from "../src/.hidden.mentee.js";

let start = true,
  testInput = [
    371910,
    769431,
    221294,
    859307,
    847617,
    348466,
    50236,
    133987,
    698570,
    822406,
    54313,
    17788,
    72083,
    649924,
    968740,
    942674,
  ],
  expected = [769431, 859307, 847617, 133987, 54313, 72083];
if (start) {
  test("Not so fast buddy!!! One more Test \n🤣\n", () =>
    expect(true).toBe(true));
  console.log(
    "One more Test \n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n\n🤣\n"
  );
} else {
  test("Make my function signagure look better", () => {
    const func = getOddNumbers(testInput);
    expect(func).toStrictEqual(expected);
  });
}
