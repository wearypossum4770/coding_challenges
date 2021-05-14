import oddishOrEvenish from "../src/oddishOrEvenish.js";
test.each([
  [43, "Oddish"],
  [373, "Oddish"],
  [55551, "Oddish"],
  [694, "Oddish"],
  [4433, "Evenish"],
  [11, "Evenish"],
  [211112, "Evenish"],
])("returns summation of digits is oddis or evenish", (num, output) => {
  let func = oddishOrEvenish(num);
  expect(func).toBe(output);
});
