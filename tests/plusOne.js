import { plusOne } from '../src/index.js'
import { expect, it } from "bun:test";

it('adds one to an array of integers', () => {
  expect(plusOne([1,2,3])).toBe([1,2,4])
})
it('adds one to an array of integers', () => {
  expect(plusOne([4,3,2,1])).toBe([4,3,2,2])
})
it('adds one to an array of integers', () => {
  expect(plusOne([9])).toBe([10])
})

// it('adds one to an array of integers', () => {
//   expect(plusOne([6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,3])).toBe([10])
// })

