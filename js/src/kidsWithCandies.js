export default function kidsWithCandies(candies, extraCandies) {
  let greatest = Math.max(...candies);
  return candies.map((kid) => kid + extraCandies >= greatest);
}

console.log(kidsWithCandies([2, 3, 5, 1, 3], 3));
