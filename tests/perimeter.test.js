import { perimeter } from '../src/index.js'
import { expect, it } from "bun:test";


it("should use branchless programing to return perimeter", () => {
  expect(perimeter("s", 1)).toBe( 4.0);
  expect(perimeter("s", 4)).toBe( 16.0);
  expect(perimeter("s", 9)).toBe( 36.0);
  expect(perimeter("s", 13)).toBe( 52.0);
  expect(perimeter("s", 30)).toBe( 120.0);
  expect(perimeter("c", 1)).toBe( 6.28);
  expect(perimeter("c", 4)).toBe( 25.12);
  expect(perimeter("c", 9)).toBe( 56.52);
  expect(perimeter("c", 13)).toBe( 81.64);
  expect(perimeter("c", 30)).toBe( 188.4);
})