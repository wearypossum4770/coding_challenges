<script setup>
import { ref, onMounted, reactive } from "vue";
import { items } from "./movies.json";
import StarIcon from "@/components/star-icon.vue";
import BaseModal from "@/components/base-modal.vue";
import useMovie from "@/hooks/use-movie.ts";
import { displayProps, showModal, handleRating } from "@/helpers.js";

onMounted(async () => {
  const options = {
    headers: { "Content-Type": "application/json" },
    mode: 'cors',
  }
  try {
    const resp = await fetch('http://localhost:3000/movies', options)
    if (resp.ok) {
      state.value = await resp.json()
    }
  } catch ({ message }) {
    console.log({ message })
  }
})
const $refs = ref(null);
const state = ref([]);
const baseModalPopover = ref(null);
const data = reactive({
  state,
  modalIsOpen: false,
  get totalMovies() { return state.value.length },
  get averageRating() { return state.value.reduce((a, b) => a + b.rating, 0) / this.totalMovies }
})

</script>

<template>
  <button @click="showModal(baseModalPopover)">Add Movie</button>
  <base-modal ref="baseModalPopover" />
  {{ `Total Movies ${data.totalMovies} / Average Rating: ${data.averageRating}` }}
  <div class="movie-app">
    <article class="movie-list" v-for="movie in state" :key="movie.id" :data-key="movie.id">
      <div class="movie-list-poster" :style="{ backgroundImage: `url(${movie.image})` }">
        <StarIcon class="five-point-star">
          <span class="five-point-star-rating">{{ displayProps(movie) }}</span>
        </StarIcon>
      </div>
      <div class="movie-list-container">
        <h3>{{ movie.name }}</h3>
        <p class="movie-genre-list">
          <span class="movie-genre" v-for="genre in movie.genres" :data-list-key="`${movie.id}-${genre}`"
            :key="`${movie.id}-${genre}`">{{ genre }}</span>
        </p>
        <p>{{ movie.description }}</p>
        <p>
          <span class="movie-list-rating">{{
            `Rating (${movie.rating} / 5)`
          }}</span>
          <button v-for="star in 5" class="movie-list-rating-button" :key="star" :data-movie-id="movie.id"
            :data-user-rating="movie.rating" @click="handleRating(movie.id, star, state)"
            :disabled="movie.rating === star" :data-star-rating="star">
            <star-icon is-button v-bind="movie" :is-active="movie.rating >= star" />
          </button>
        </p>
      </div>
    </article>
  </div>
</template>
