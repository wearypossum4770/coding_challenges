import circuitPower from '../src/circuitPower.js'

test.each([[[110, 3], 330],
    [[230, 10], 2300],
    [[480, 20], 9600],])("test should calculated resistance", (testInput, testOutput)=>{
        let func = circuitPower(testInput)
        expect(func).toBe(testOutput)
    })