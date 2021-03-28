import removeLeadingTrailing from "../src/removeLeadingTrailing.js";

test.each([
  ["230.000", "230"],
  ["00402", "402"],
  ["03.1400", "3.14"],
  ["30", "30"],
  ["30.0000", "30"],
  ["24340", "24340"],
  ["0404040", "404040"],
  ["0", "0"],
  ["00", "0"],
  ["0.000000", "0"],
  ["0000.000", "0"],
  ["00.0001", "0.0001"],
  ["10000", "10000"],
  ["1345", "1345"],
  ["30.000020", "30.00002"],
  ["00200.1900001", "200.1900001"],
])(
  "remove leading and trailing zeros from a number string",
  (testInput, testOutput) => {
    let func = removeLeadingTrailing(testInput);
    expect(func).toBe(testOutput);
  }
);
