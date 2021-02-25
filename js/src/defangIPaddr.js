export default function defangIPaddr(address) {
  let newAddress = "";
  for (let index = 0; index < address.length; index++) {
    address[index] !== "."
      ? (newAddress += address[index])
      : (newAddress += `[${address[index]}]`);
  }
  return newAddress;
}
