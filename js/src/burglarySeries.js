export function calculateLosses(obj) {
  if (obj.constructor === Object && Object.keys(obj).length > 0) {
    return Object.values(obj).reduce((accum, currVal) => accum + currVal);
  } else {
    return "Lucky you!";
  }
}
export function mostExpensiveItem(obj) {
  let most = Math.max(...Object.values(obj));
  for (let prop in obj) {
    if (obj[prop] === most) {
      return prop;
    }
  }
}
export function findIt(obj, name) {
  let keyArray = Object.keys(obj);
  return keyArray.includes(name)
    ? `${name[0].toUpperCase()}${name.slice(1)} is gone...`
    : `${name[0].toUpperCase()}${name.slice(1)} is here!`;
}
