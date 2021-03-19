const unsortedList = ["Mary", "alfred", "John"];

export default function sortedStringList(array) {
  return array.sort((a, b) => a.localeCompare(b));
}

console.log(sortedStringList(unsortedList));
