import addition from '../src/addition.js'


test.each([[[100, 1000], 1100],[[2,3],5]])("correctly adds numbers", (test_input, expected)=>{
	const func = addition(..test_input)
	expect(func).toEqual(expected)
})

