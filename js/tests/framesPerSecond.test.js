import framesPerSecond from "../src/framesPerSecond.js";
test.each([
  [[1, 1], 60],
  [[10, 1], 600],
  [[10, 25], 15000],
  [[500, 60], 1800000],
  [[0, 60], 0],
  [[99, 1], 5940],
  [[419, 70], 1759800],
  [[52, 33], 102960],
])("shoudl work", (minutes, fps, output) => {
  let func = framesPerSecond(minutes, fps);
  expect(func).toBe(output);
});
