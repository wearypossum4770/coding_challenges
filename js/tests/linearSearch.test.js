import {linearSearch,linearSearchGlobal} from '../src/linearSearch.js'
test.each([
[3, [1,2,3], 2],
[4, [1,2,3], undefined],
[13, [1,2,3], undefined],
])("deep search", (testInput1, testInput2, testOutput)=>{
let func = linearSearch(testInput1, testInput2)
expect(func).toBe(testOutput)
})


test.each([
    ["a", ["b", "a", "n", "a", "n", "a", "s"], [1, 3, 5]],
["s", ["b", "a", "n", "a", "n", "a", "s"], [6]],
["n", ["b", "a", "n", "a", "n", "a", "s"], [2, 4]],

])("shallow search", (testInput1, testInput2, testOutput)=>{
    let func = linearSearchGlobal(testInput1, testInput2)
expect(func).toStrictEqual(testOutput)

})
