import isRepdigit from "../src/isRepdigit.js";
test.concurrent.each([
  [6, true],[66, true],[666, true],[6666, true],[1001, false],[
  -11, false, "The number must be >= 0"
  ],
])("is a rep digit", async (param, output) => {
  const func = isRepdigit(param);
  expect(func).toBe(output);
});
