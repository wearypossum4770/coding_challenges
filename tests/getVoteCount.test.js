import { describe, expect, it } from "bun:test";

import getVoteCount from '../src/getVoteCount.js'
describe('Summation of vote totals', () => {
  it('calculates the correct total', () => {
    expect(getVoteCount({ upvotes: 13, downvotes: 0 })).toBe( 13)
    expect(getVoteCount({ upvotes: 2, downvotes: 33 })).toBe( -31)
    expect(getVoteCount({ upvotes: 132, downvotes: 132 })).toBe( 0)
    expect(getVoteCount({ upvotes: 0, downvotes: 0 })).toBe( 0)
    expect(getVoteCount({ downvotes: 4, upvotes: 1 })).toBe( -3)
  })
});