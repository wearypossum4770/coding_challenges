const input = "is2 sentence4 This1 a3"
const output = "This is a sentence"
const organizeSentence = (sentence) => {
    let arr = sentence.split(" ");
    arr.sort((a,b) => a[a.length-1] - b[b.length-1])
    arr = arr.map((word) => word.slice(0, word.length-1))
    sentence = arr.join(' ');
    return sentence
}
console.log(organizeSentence(input) === output)