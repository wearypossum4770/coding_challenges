import { reverseInteger } from '../src/index.js';
import { expect, it } from "bun:test";

it("should reverse and integer", () => {
  expect(reverseInteger(123)).toBe( 321)
expect(reverseInteger(-123 )).toBe(-321)
expect(reverseInteger(120)).toBe( 21)
})