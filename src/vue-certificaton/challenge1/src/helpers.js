import { unref } from "vue";
import fetchOptions from '@/assets/data/fetch-options.json'
export const showModal = ($refs) => $refs.baseModalPopover.showModal();

export const closeModal = ($refs) => $refs.close("Movie Not added");

export const displayProps = ({ rating }) => (rating > 0 ? rating : "-");

export const handleRating = (movieId, star, state) =>
  (unref(state)[movieId].rating = star);

export const merge = (...args) => Object.assign({}, ...args)

export const stringify = (...args) => JSON.stringify(merge({}, ...args))

export const addMovie = ({
  studio = "",
  releaseDate = "",
  director = "",
  name = "",
  image = "",
  rating = 0,
  description = "",
  yearPublished = 0,
  genre = [],
  genres = [],
  writer = "",
  cast = [],
}) => ({
  rating,
  studio,
  releaseDate,
  director,
  name,
  image,
  description,
  yearPublished,
  genres: genre || genres,
  writer,
  cast,
});

const ensureArray = (arr) => [arr].flat(Infinity)
/**
 * 
 * @param { { string: string|number|boolean }[] } array 
 * @param { number[] } keys 
 * @returns { { string: string|number|boolean }[] }
 */
export const removeObjectById = (array, keys=[]) => {
  const reject = new Set(keys)
  return ensureArray(array).reduce((a,b) => reject.has(b.id) ? a: [...a, b],[])
}
/**
 * 
 * @param { Array.<string|number|boolean> } array 
 * @param { number[] } keys 
 * @returns { Array.<string|number|boolean> }
 */

export const drainArrayByIndex = (array, keys = []) => {
  const reject = new Set(keys)
  return ensureArray(array).reduce((a, b, i) => reject.has(i) ? a : [...a, b],[])
}

export const arrayItemSwap = (array, index, item) => ensureArray(array).reduce((a, b, i) => i === index ? [...a, item]: [...a, b],[])


export const initialState = async (options) => {
try {
  return {state: await (await fetch(new URL('http://localhost:3000/movies'), merge(fetchOptions, options))).json()}
} catch ({ message }) {
  return { message }
}
}