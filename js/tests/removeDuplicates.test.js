import removeDuplicates from '../src/easy/removeDuplicates.js'

test.each(

    [
        [[1,1,2], [1,2]],
       [ [0,0,1,1,1,2,2,3,3,4], [0,1,2,3,4]],
       [[1,1,1,1], [1]]
    ]
)

("duplicates removed", (data,expected)=>{
    expect(removeDuplicates(data)).toStrictEqual(expected)
})