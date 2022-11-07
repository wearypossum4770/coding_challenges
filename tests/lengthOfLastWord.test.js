import { lengthOfLastWord } from '../src/index.js'
import { expect, it } from "bun:test";

it('return legnth of last word in a string', () => expect(lengthOfLastWord("Hello World")).toBe(5))
it('return legnth of last word in a string', () => expect(lengthOfLastWord("   fly me   to   the moon  ")).toBe(4))
it('return legnth of last word in a string', () => expect(lengthOfLastWord("luffy is still joyboy")).toBe(6))
