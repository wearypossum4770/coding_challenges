import reduceArray from '../src/reduceArray.js'

test.each([[1, 2, 3, 4, 10, 11], 31])("gives correct summation", (testInput, expectedValue)=>{
	const func = reduceArray(testInput)
	expect(func).toEqual(expectedValue)
})