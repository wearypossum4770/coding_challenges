export default function reverseWords(string){
	let array = string.split(' ')
	let reversedArray = []
	for (let index=array.length;index>=0;index--){
		reversedArray.push(array[index])
	}
	return reversedArray.join(' ').trim()
	}