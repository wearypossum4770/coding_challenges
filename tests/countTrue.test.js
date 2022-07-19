import {countTrue} from '../src/index.js';
import { expect, it } from "bun:test";
it('counts the number of true values in an array', () => {
expect(countTrue([true, false, false, true, false])).toBe( 2)
expect(countTrue([false, false, false, false])).toBe( 0)
expect(countTrue([])).toBe( 0)
expect(countTrue([false, false, true, true, false, false, false, true, true, true, true, false, true, true, false])).toBe( 8)
expect(countTrue([true, false, true, true, false, false, false, false, false])).toBe( 3)
expect(countTrue([false, true, true, false, true, true, false, true, false, true, false, true, false, true, false])).toBe( 8)
expect(countTrue([true, false, true, true, true, false, true, true, false, false])).toBe( 6)
expect(countTrue([false, false, false, false, true, false, true, false, true, false, false])).toBe( 3)
expect(countTrue([true, false, false, false, true, false, false, true, false, false, false])).toBe( 3)
expect(countTrue([true, true, false, true, false, false, false, false, true, false])).toBe( 4)
expect(countTrue([true, false, true, true, false, true, true, true, true, false, true, false, true, false])).toBe( 9)
expect(countTrue([true, false, true, true, true, true, false, true, true, false, true, false, false, false, false])).toBe( 8)
expect(countTrue([true, true, false, false, false, false, true, false, true, true, false, true])).toBe( 6)
})

