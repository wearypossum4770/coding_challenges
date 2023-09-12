<script setup lang="ts">
import { ref } from 'vue'
import { items } from "./movies.json";

/*
 This is an Icon that you can use to represent the stars if you like
 otherwise you could just use a simple ⭐️ emoji, or * character.
*/
// import { StarIcon } from "@heroicons/vue/24/solid";
</script>

<template>
  <div class="movie-app">
    <article class="movie-list" v-for="movie in items">
      <img :src="movie.image" width="300" />
      <div class="movie-list-container">
        <h3>{{ movie.name }}</h3>
        <p class="movie-genre-list"><span class="movie-genre" v-for="genre in movie.genres">{{ genre }}</span></p>
        <p>{{ movie.description }}</p>
        <p>{{ `Rating (${movie.rating} / 5)` }}&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;{{ '⭐️'.repeat(movie.rating) }}</p>
      </div>
    </article>
  </div>
</template>
