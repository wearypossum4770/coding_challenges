export default function sortDescending(num) {
  return parseInt(
    Array.from(num.toString())
      .sort((a, b) => b - a)
      .join("")
  );
}
