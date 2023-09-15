<template>
  <Transition name="default-modal">
    <dialog ref="baseModalPopover" class="create-movie-form" :id="modal.id">
      <form ref="createMovieForm" @submit.prevent id="create-movie-form">
        <h1>New Movie</h1>
        <datalist id="genre-choice-list">
          <option v-for="genre in genres" :key="genre.id" data-genre-value="genre.code" :value="genre.genre">
            {{ genre.genre }}
          </option>
        </datalist>
        <div class="new-movie-form">
          <label> Name </label>
          <input name="movieName" required v-model="formData.name" />
        </div>
        <div class="new-movie-form">
          <label>Description</label>
          <textarea v-model="formData.description" name="description"></textarea>
        </div>
        <div class="new-movie-form">
          <label>Image</label>
          <input type="url" v-model="formData.image" name="image" />
        </div>
        <div class="new-movie-form">
          <label>Genres</label>
          <input type="search" v-model="formData.genre" list="genre-choice-list" id="genre-choice" name="genres" />
        </div>
        <div class="new-movie-form">
          <label>In Theatres?</label>
          <input type="checkbox" name="inTheaters" v-model="formData.inTheaters" />
        </div>
        <footer class="new-movie-form-footer">
          <button type="reset" id="cancel-button">Cancel </button>
          <button id="save-button" type="submit" form="create-movie-form" @click="handleSubmit">
            Save
          </button>
        </footer>
      </form>
    </dialog>
  </Transition>
</template>

<script setup>
import { ref, onMounted } from "vue";
import genres from "@/assets/data/genre-database.json";
import useModal from "@/stores/modal.js";
import mulligans from '@/mulligans-data.json';

const modal = useModal();
const baseModalPopover = ref(null);
const createMovieForm = ref(null);
const formData = ref({
  errors: [],
  get hasErrors() { return this.errors.length > 1 },
  name: "",
  rating: 0,
  image: "",
  inTheaters: false,
  genre: "",
  description: "",
});

const validationRules = rule => {
  if (rule === 'required') return /^ *$/;
  return null
}


const handleSubmit = async ({ target: { form } }) => {
  const payload = { ...formData.value.entries(), ...mulligans }
  const options = {
    mode: 'cors',
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  }
  try {
    const resp = await fetch(new URL('http://localhost:3000/movies/new'), options)
    Object.assign(payload, resp)
    if (resp.ok) {
      const data = await resp.json()
      Object.assign(payload, data)
    }
  } catch ({ message, name }) {
    Object.assign(payload, { message, name })
  } finally {
    return payload
  }
};

defineExpose({ baseModalPopover, createMovieForm });
const props = defineProps(["baseModalPopover"]);
onMounted(() => {
  formData.value = new FormData(createMovieForm.value);
});
</script>
