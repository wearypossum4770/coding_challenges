const months = {
  1: "A",
  2: "B",
  3: "C",
  4: "D",
  5: "E",
  6: "H",
  7: "L",
  8: "M",
  9: "P",
  10: "R",
  11: "S",
  12: "T",
};

function fiscalCode(person) {
  let target = ``;
  const consonants = /[^aeiou]/gi;
  const vowels = /[aeiou]/gi;
  function partOne({ surname }) {
    if (surname.length < 3) {
      return `${surname}X`;
    } else {
      let part = surname.match(consonants);
      if (part.length >= 3) {
        return part.join("");
      } else {
        return [...part, surname.match(vowels)[0]].join("");
      }
    }
  }
  function partTwo({ name }) {
    let cPart = name.match(consonants);
    let vPart = name.match(vowels)[0];
    if (name.length < 3) {
      return `${cPart[0]}${vPart[0]}X`;
    } else {
      if (cPart.length === 3) {
        return cPart.join("");
      } else if (cPart.length > 3) {
        return `${cPart[0]}${cPart[2]}${cPart[3]}`;
      } else {
        return [...cPart, vPart[0]].join("");
      }
    }
  }
  function partThree({ dob, gender }) {
    let [day, month, year] = dob.split("/");
    let ending = "";
    if (gender === "M") {
      ending += parseInt(day) < 10 ? `0${day}` : day;
    } else {
      ending += parseInt(day) + 40;
    }

    return year.slice(-2) + months[parseInt(month)] + ending;
  }
  return (
    target +
    partOne(person).toUpperCase() +
    partTwo(person).toUpperCase() +
    partThree(person)
  );
}

console.log(
  fiscalCode({
    name: "Brendan",
    surname: "Eich",
    gender: "M",
    dob: "1/12/1961",
  })
); //=== "CHEBND61T01")
console.log(
  fiscalCode({ name: "Helen", surname: "Yu", gender: "F", dob: "1/12/1950" })
); //=== "YUXHLN50T41")
console.log(
  fiscalCode({ name: "Al", surname: "Capone", gender: "M", dob: "17/1/1899" })
); //=== "CPNLAX99A17")
console.log(
  fiscalCode({
    name: "Mickey",
    surname: "Mouse",
    gender: "M",
    dob: "16/1/1928",
  })
); //=== "MSOMKY28A16")
console.log(
  fiscalCode({ name: "Marie", surname: "Curie", gender: "F", dob: "7/11/1867" })
); //=== "CRUMRA67S47")
