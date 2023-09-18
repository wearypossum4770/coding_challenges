<script setup>
// https://developer.chrome.com/blog/introducing-popover-api/
// https://developer.mozilla.org/en-US/docs/Web/CSS/::backdrop
// https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close
import { ref, onMounted, reactive, unref } from "vue";
import { items } from "@/movies.json";
import genres from "@/assets/data/genre-database.json";

import useModal from "@/stores/modal.js";
import StarIcon from "@/components/star-icon.vue";
import useMovie from "@/hooks/use-movie.ts";
import {
  displayProps,
  showModal,
  initialState,
  handleRating,
  drainArrayByIndex,
  postData,
  arrayPush,
} from "@/helpers.js";
const modal = useModal();
const baseModalPopover = ref(null);

const handleSubmit = ({ target: { value, id } }) => {
  console.log({ value, id });
  if (!modal.editMode) {
    modal.$patch({ state: arrayPush(modal.state, modal.partialState) });
    postData({ data: modal.partialState });
  }
  baseModalPopover.value.close("movie list updated");
  modal.resetPartialState();
};
onMounted(async () => modal.$patch(await initialState()));



const resetRatings = async () =>
  modal.$patch({ state: modal.state.map((a) => ({ ...a, rating: 0 })) });
const enterNewMode = () => {
    modal.editMode  = false
  showModal(unref(baseModalPopover))
}
const enterEditMode =  (id) => {
  modal.$patch({
    partialState: modal.state[id],
    editMode: true,
  });
  unref(baseModalPopover).showModal();
};
const removeFromList = (id, movieId) =>
  modal.$patch({ state: drainArrayByIndex(modal.state, [id]) });

const closeModal = (event) => {
  console.log(event);
  baseModalPopover.value.close("modal manually closed");
};
</script>

<template>
  <aside class="movie-app-action-menu">
    <span>{{
      `Total Movies ${
        modal.totalMovies
      } / Average Rating: ${modal.averageRating.toFixed(1)}`
    }}</span>
    <div>
      <button @click="enterNewMode">Add Movie</button>
      <button @click="resetRatings">Reset Ratings</button>
    </div>
  </aside>
  <div class="movie-app">
    <article
      class="movie-list"
      v-for="(movie, id) in modal.state"
      :key="movie.id"
      :data-key="id"
    >
      <div
        class="movie-list-poster"
        :style="{ backgroundImage: `url(${movie.image})` }"
      >
        <StarIcon class="five-point-star">
          <span class="five-point-star-rating">{{ displayProps(movie) }}</span>
        </StarIcon>
      </div>
      <div class="movie-list-container">
        <h3>{{ movie.name }}</h3>
        <p class="movie-genre-list">
          <span
            class="movie-genre"
            v-for="genre in movie.genres"
            :data-list-key="`${movie.id}-${genre}`"
            :key="`${movie.id}-${genre}`"
            >{{ genre }}</span
          >
        </p>
        <p>{{ movie.description }}</p>
        <div class="movie-list-actionbar">
          <button @click="enterEditMode(id)">
            <img height="15" src="@/assets/images/icons/pen-solid.svg" />
          </button>
          <button @click="removeFromList(id)">
            <img height="15" src="@/assets/images/icons/trash-solid.svg" />
          </button>
        </div>
        <p>
          <span class="movie-list-rating">{{
            `Rating (${movie.rating} / 5)`
          }}</span>
          <button
            v-for="star in 5"
            class="movie-list-rating-button"
            :key="star"
            :data-movie-id="movie.id"
            :data-user-rating="movie.rating"
            @click="handleRating(id, star, modal.state)"
            :disabled="movie.rating === star"
            :data-star-rating="star"
          >
            <star-icon
              is-button
              v-bind="movie"
              :is-active="movie.rating >= star"
            />
          </button>
        </p>
      </div>
    </article>
  </div>
  <Transition name="default-modal">
    <dialog ref="baseModalPopover" class="create-movie-form" :id="modal.id">
      <form @submit.prevent id="create-movie-form">
        <h1>New Movie</h1>
        <div class="new-movie-form">
          <label> Name </label>
          <input
            id="movie-name"
            name="name"
            autofocus
            v-model="modal.partialState.name"
          />
        </div>
        <div class="new-movie-form">
          <label>Description</label>
          <textarea
            v-model="modal.partialState.description"
            name="description"
          ></textarea>
        </div>
        <div class="new-movie-form">
          <label>Image</label>
          <input type="url" v-model="modal.partialState.image" name="image" />
        </div>
        <div class="new-movie-form">
          <label>Genres</label>
          <select
            v-model="modal.partialState.genres"
            id="genre-choice"
            name="genres"
            multiple
          >
            <option
              v-for="genre in genres"
              :key="genre.id"
              :selected="modal.partialState.genres.includes(genre)"
              data-genre-value="genre.code"
              :value="genre.genre"
            >
              {{ genre.genre }}
            </option>
          </select>
        </div>
        <div class="new-movie-form">
          <label>In Theatres?</label>
          <input
            type="checkbox"
            name="inTheaters"
            v-model="modal.partialState.inTheaters"
          />
        </div>
        <footer class="new-movie-form-footer">
          <button
            @click="closeModal"
            type="button"
            popovertargetaction="hide"
            :popovertarget="modal.id"
            id="cancel-button"
          >
            Cancel
          </button>
          <button
            id="save-button"
            type="submit"
            form="create-movie-form"
            @click="handleSubmit"
          >
            Save
          </button>
        </footer>
      </form>
    </dialog>
  </Transition>
</template>
