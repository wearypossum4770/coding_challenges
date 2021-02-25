import findDuplicates from "../src/findDuplicates.js";
import { HUGE_ARRAY } from "../src/oh.js";
import { data } from "../data/findDuplicates.data.js";
test("It passes", () => {
  let func = findDuplicates(HUGE_ARRAY).sort((a, b) => a - b);
  expect(func).toStrictEqual(data);
});
