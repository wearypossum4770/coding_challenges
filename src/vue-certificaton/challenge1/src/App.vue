<script setup>
import { ref } from "vue";
import { items } from "./movies.json";
import StarIcon from "@/components/star-icon.vue";

const state = ref(items.map((i) => ({ ...i, userRating: 0, isDisabled: false })));
const handleRating = (movieId, star) => {
  const movie = state.value[movieId - 1]
  console.log(movie)
  // const { userRating, rating } = movie
  movie.rating = star
  // console.log({ userRating: starRating, rating: starRating, isDisabled: true })

  // if (userRating === starRating) return Object.assign(movie, { isDisabled: true })
  // return Object.assign(movie, { userRating: starRating, rating: starRating, isDisabled: true })
};
/*
 This is an Icon that you can use to represent the stars if you like
 otherwise you could just use a simple ⭐️ emoji, or * character.
*/
// import { StarIcon } from "@heroicons/vue/24/solid";
</script>

<template>
  <div class="movie-app">
    <article class="movie-list" v-for="movie in state" :key="movie.id" :data-key="movie.id">
      <img :src="movie.image" width="300" alt="" />
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
            :data-user-rating="movie.rating" @click="handleRating(movie.id, star)" :disabled="movie.rating === star"
            :data-star-rating="star">
            <star-icon :is-active="movie.rating >= star" />
          </button>
        </p>
      </div>
    </article>
  </div>
</template>
