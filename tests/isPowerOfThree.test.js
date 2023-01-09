import { isPowerOfThree } from "./index.js";
import { expect, it } from "bun:test";

it("finds cube roots", () => {
  expect(isPowerOfThree(27)).toBe(true);
  expect(isPowerOfThree(0)).toBe(false);
  expect(isPowerOfThree(9)).toBe(true);
});
