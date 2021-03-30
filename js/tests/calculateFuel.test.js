import calculateFuel from '../src/calculateFuel.js'

test.each([
    [15, 150],
[23, 230],
[10, 100],
[3, 100],
[23.5, 235],
[3.14, 100],
[9.99999, 100],
[822315322, 8223153220],
[12345.6789, 123456.789],
[31.41, 314.1],
])("should calculate correct gas", (testInput, testOutput)=>{
    let func = calculateFuel(testInput)
    expect(func).toBe(testOutput)
})

// Author: Joshua Señoron