import reverseWords from "../src/reverseWords.js"

test("someting",()=>{
	const expected="blue is sky the"
	const func = reverseWords("the sky is blue")
	expect(func).toStrictEqual(expected)
	
	})
