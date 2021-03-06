function stripProperty(obj, prop) {
  let init = Object.entries(obj);
  let object = {};
  init.forEach((item) => {
    let [key, value] = item;
    if (key !== prop) {
      object[key] = value;
    }
  });

  // write your code here

  return object;
}
async function getTeams(year, k) {
  // write your code here
  // API endpoint template: https://jsonmock.hackerrank.com/api/football_matches?competition=UEFA%20Champions%20League&year=<YEAR>&page=<PAGE_NUMBER>
  const request = (
    await fetch(
      `https://jsonmock.hackerrank.com/api/football_matches?competition=UEFA%20Champions%20League&year=${year}&page=${page}`
    )
  ).json();
  return request;
}

console.log(getTeams(2011, 1));
