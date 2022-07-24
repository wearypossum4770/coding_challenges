const perimeter = (shape, value) => {
  const factor = new Map([
    ["s", 4.0],
    ["S", 4.0],
    ["C", 6.28],
    ["c", 6.28],
  ]).get(shape);
  return factor * value;
};

export default perimeter;
