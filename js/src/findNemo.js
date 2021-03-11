export default function findNemo(sentence){
let cleanedSentence = sentence.split(' ')
for (let index = 0; index<cleanedSentence.length;index++){
    if (cleanedSentence[index]==='Nemo'){   
        return `I found Nemo at ${index+1}!`
    }
}
return "I can't find Nemo :("
}

console.log(findNemo("I am Ne mo Nemo !"))