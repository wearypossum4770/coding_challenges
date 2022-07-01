/**
 *
 * @param {number|bigint} sample_size  - sample/population  size (n)
 * @returns
 */
export default function sturgesRule(sample_size) {
  return 1 + 3.3 * Math.log10(sample_size);
}
/**
 *
 * @param {number|bigint} min - minimum sample/population  value
 * @param {number|bigint} max - maximum sample/population  value
 * @param {number|bigint} sample_size  - sample/population  size (n)
 * @returns {number|bigint} - width of bins for the sample/population
 */
export function bin_width(min, max, sample_size) {
  return (max - min) / sample_size;
}
console.log(sturgesRule(44));
