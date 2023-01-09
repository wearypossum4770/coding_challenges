import { longestCommonPrefix } from "./index.js";
import { expect, it } from "bun:test";

it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix([])).toBe("")
});
it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix(['dog'])).toBe("dog")
});
it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix(["apple", "apply", "ape", "at"])).toBe("a")
});
it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix(["dog", "racecar", "car"])).toBe("")
});
it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix(["flower", "flower", "flower", "flower"])).toBe("flower")
});
it("should return the longest prefix", () =>  {
  expect(longestCommonPrefix(["flower", "flow", "flight"])).toBe("fl")
});