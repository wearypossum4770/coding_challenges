const arrayToSearchThrough = [];
for (let i = 1; i <= 1000; i++) {
  arrayToSearchThrough.push(i);
}

export function linearSearch(valueToFind, arrayToSearchThrough) {}

export function linearSearchGlobal(valueToFind, arrayToSearchThrough) {}

/**const arrayToSearchThrough = [];
for (let i = 1; i <= 1000; i++) {
    arrayToSearchThrough.push(i);
}

exports.linearSearch = function(valueToFind, arrayToSearchThrough) {

    if (!arrayToSearchThrough.includes(valueToFind)) {
        return undefined
    } else {
        for (let i = 0; i < arrayToSearchThrough.length; i++) {
            return arrayToSearchThrough.indexOf(valueToFind)
        }
    }
};
exports.linearSearchGlobal = function(valueToFind, arrayToSearchThrough) {
    let keyValePairGlobal = []

    for (let j = 0; j < arrayToSearchThrough.length; j++) {

        if (valueToFind == arrayToSearchThrough[j]) {
            keyValePairGlobal.push(j)
        }
    }
    return keyValePairGlobal
}

// FINALLY!!! */
