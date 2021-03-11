import remainder from '../src/remainder.js'
test.each([[ [7, 2], 1],
    [[3, 4], 3],
    [[-9, 45], -9],
    [[5, 5], 0],
    [[1, 3], 1],
    [[3, 4], 3],
    [[-9, 45], -9],
    [[5, 5], 0],])("", (testInput, funcOutput) => {
    let func = remainder(testInput);
    expect(func).toEqual(funcOutput);
  })