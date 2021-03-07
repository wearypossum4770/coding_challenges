import {alphabetSoup} from '../src/alphabetSoup.js'

test.each([        ["hello","ehllo"],
["edabit","abdeit"],
["hacker","acehkr"],
["geek","eegk"],
["javascript","aacijprstv"],
["credibility","bcdeiiilrty"],
["apostrophe","aehoopprst"],
["determination","adeeiimnnortt"],
["win","inw"],
["synthesis","ehinsssty"],])("The string %s in alphabetical order is %s", (testInput, funcOutput)=>{
    let func = alphabetSoup(testInput)
    expect(func).toStrictEqual(funcOutput)
})