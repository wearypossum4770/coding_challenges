import findNemo from '../src/findNemo.js'
test.each([
    ["I am Ne mo Nemo !", "I found Nemo at 5!"],
["N e m o is NEMO NeMo Nemo !", "I found Nemo at 8!"],
["I am Nemo's dad Nemo senior .", "I found Nemo at 5!"],
["Oh, hello !", "I can't find Nemo :("],
["Is it Nemos, Nemona, Nemoor or Garfield?", "I can't find Nemo :("],
["Nemo is a clown fish, he has white and orange stripes. Nemo , come back!", "I found Nemo at 1!"],
["I am finding Nemo !",  "I found Nemo at 4!"],
["Nemo is me",  "I found Nemo at 1!"],
["I Nemo am",  "I found Nemo at 2!"],
])("function finds the word 'Nemo' is case-sensitive", (testInput, funcOutput) => {
    let func = findNemo(testInput);
    expect(func).toEqual(funcOutput);
  } )