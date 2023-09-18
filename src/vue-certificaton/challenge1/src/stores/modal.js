import { defineStore, acceptHMRUpdate } from "pinia";
import { unref, ref, reactive, computed } from "vue";
import { arrayItemSwap } from "@/helpers";
const initialPartialState = () => ({
  id: 0,
  name: "",
  description: "",
  image: "",
  rating: 0,
  genres: [],
  inTheaters: false,
});
/**
 * @type StoreDefinition
 * @name ModalStoreDefinition
 */
const store = defineStore("modal", () => {
  const state = ref([]);

  const partialState = ref(initialPartialState());

  const totalMovies = computed(() => unref(state).length ?? 0);
  const averageRating = computed(
    () => unref(state).reduce((a, b) => a + b.rating, 0) / unref(totalMovies),
  );
  return {
    id: ref("add-movie-modal"),
    editMode: ref(false),
    partialState,
    state,
    totalMovies,
    $reset: () => (state.value = []),
    resetPartialState: () => (partialState.value = initialPartialState()),
    averageRating,
  };
});
export default store;
if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(store, import.meta.hot));
}
