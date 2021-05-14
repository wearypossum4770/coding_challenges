import nthEven from "../src/nthEven.js";
test.concurrent.each`
intput | output
${1} | ${0}
${2} | ${2}
${3} | ${4}
${100} | ${198}
${1298734} | ${2597466}

`("The $input(th) even number is $output", async ({ input, output }) => {
  expect(nthEven(intput)).toBe(output);
});
