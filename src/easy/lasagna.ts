// @ts-check
//
// ☝🏽 The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion on the web
// and supported IDEs when implementing this exercise. You don't need to
// understand types, JSDoc, or TypeScript in order to complete this JavaScript
// exercise, and can completely ignore this comment block and directive.

// 👋🏽 Hi there!
//
// On the JavaScript track we provide you with stubs. These stubs provide a
// starting point to solving the exercise.
//
// In general, each variable/constant and each declared function will have a
// JSDoc comment block above it, explaining what the variable/constant holds or
// the function is supposed to accomplish.
//
// 💡 Often, the JSDoc comment blocks have annotations, such as @param and
// @returns which are usually highlighted with a different color if the IDE
// you're in recognizes them. It's these annotations that are used when
// referring to the constant, variable, or function from somewhere else that
// IDEs display.
//
// You don't need to write these yourself; it is not expected in idiomatic
// JavaScript, but some companies and style-guides do enforce them.
//
// 💡 You're allowed to completely clear a stub before you get started. Often
// we recommend using the stub, because they are already set-up correctly to
// work with the tests, which you can find in ./lasagna.spec.js
//
// Good luck preparing some lasagna!

/**
 * The number of minutes it takes to prepare a single layer.
 */
export const PREPARATION_MINUTES_PER_LAYER = 2;
/**
 * The nmber of minutes it takes to bake in the oven.
 */
export const EXPECTED_MINUTES_IN_OVEN = 40;
/**
 * Determines the number of minutes the lasagna still needs to remain in the
 * oven to be properly prepared.
 *
 * @param  actualMinutesInOven
 * @returns  the number of minutes remaining
 */
export const remainingMinutesInOven = (actualMinutesInOven: number) => EXPECTED_MINUTES_IN_OVEN  - actualMinutesInOven

/**
 * Given a number of layers, determines the total preparation time.
 *
 * @param  numberOfLayers
 * @returns  the total preparation time
 */
export const preparationTimeInMinutes = (numberOfLayers: number)  =>  PREPARATION_MINUTES_PER_LAYER * numberOfLayers

/**
 * Calculates the total working time. That is, the time to prepare all the layers
 * of lasagna, and the time already spent in the oven.
 *
 * @param  numberOfLayers
 * @param  actualMinutesInOven
 * @returns  the total working time
 */
export const totalTimeInMinutes = (numberOfLayers: number , actualMinutesInOven: number)  => actualMinutesInOven + preparationTimeInMinutes(numberOfLayers)


// [[3,20, 26][1, 5, 7],[4, 15, 23],[1, 35, 37],].map(([layers, minutes, output]: Array<number>) => totalTimeInMinutes(layers, minutes) === output)
// [[2, 4]].map((layers, output) => preparationTimeInMinutes(layers) === output)
// [[30,10]].map((minutes, output) => remainingMinutesInOven(minutes) === output)


// console.log(totalTimeInMinutes(3, 20))


console.log(totalTimeInMinutes(1, 5), 7);
    // console.log(totalTimeInMinutes(4, 15), 3);
    // console.log(totalTimeInMinutes(1, 35), 7);