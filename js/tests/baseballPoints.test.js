import baseballPoints from '../src/baseballPoints.js'

test.each([[[1, 1], 5],
    [[7, 5], 29],
    [[38, 8], 100],
    [[1, 1], 5],
    [[1, 2], 8],
    [[2, 1], 7],
    [[2, 2], 10],
    [[69, 420], 1398],])("baseball points are calculated", (testInput, testOutput)=> {
        let func = baseballPoints(testInput)
        expect(func).toBe(testOutput)
    })