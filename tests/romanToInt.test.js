import { describe, expect, it } from "bun:test";
import {romanToInt} from "../src/index.js";

describe('roman numeral converter', () => {
  it('should give the correct value', () => {
    expect(romanToInt("III")).toBe(3)
expect(romanToInt("LVIII")).toBe(58)
expect(romanToInt("MCMXCIV")).toBe(1994)
expect(romanToInt("VII")).toBe(7)
expect(romanToInt("DCLXXIX")).toBe(679)
expect(romanToInt("MMMCMV")).toBe(3905)
expect(romanToInt("I")).toBe(1)
expect(romanToInt("VIII")).toBe(8)
expect(romanToInt("XXIX")).toBe(29)
expect(romanToInt("LIV")).toBe(54)
expect(romanToInt("CCV")).toBe(205)
expect(romanToInt("CDXLIV")).toBe(444)
expect(romanToInt("CMXCIX")).toBe(999)
expect(romanToInt("M")).toBe(1000)
expect(romanToInt("MMMDCCCLXXXVIII")).toBe(3888)
expect(romanToInt("MMMCMX")).toBe(3910)
  })
})