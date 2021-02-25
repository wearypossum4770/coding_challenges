import defangIPaddr from "../src/defangIPaddr.js";

test.each([
  ["1.1.1.1", "1[.]1[.]1[.]1"],
  ["255.100.50.0", "255[.]100[.]50[.]0"],
])("Defang Ip Address", (data, expected) => {
  expect(defangIPaddr(data)).toEqual(expected);
});
