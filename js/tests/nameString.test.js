import nameString from '../src/nameString.js'

test.each([
    
["Mubashir", "MubashirEdabit"],
["Matt", "MattEdabit"],
["javaScript", "javaScriptEdabit"],
["Airforce", "AirforceEdabit"],
])("concatenates the string", (testInput, testOutput)=> {
    let func = nameString(testInput)
    expect(func).toBe(testOutput)
})
