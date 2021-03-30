
import Guitar from '../src/destructureClasses.js'
// jest.mock('../src/destructureClasses'); 
let guitar = new Guitar()
// beforeEach(() => {
//     // Clear all instances and calls to constructor and all methods:
//     Guitar.mockClear();
//   });

describe("Destructuring Javascript Class", ()=>{
    it("is an instance of guitar class", ()=>{
expect(guitar).toBeInstanceOf(Guitar)
    })
})

// console.log(b)
// const { strings, guitarProps } = new Guitar()

// strings(true)

// console.log(guitarProps) // { strings: true }