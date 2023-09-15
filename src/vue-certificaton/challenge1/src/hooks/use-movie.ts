import { ref } from "vue";
import { items } from "@/movies.json";

export type Movie = {
  id: number;
  name: string;
  description: string;
  image: URL;
  rating: number;
  genres: string[];
  inTheaters: boolean;
};
const attrs = { userRating: 0, isDisabled: false };
const useMovie = () => ref(items.map((i: Movie) => ({ ...i, ...attrs })));
export default useMovie;
