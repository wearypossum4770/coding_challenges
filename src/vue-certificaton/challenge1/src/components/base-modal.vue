<template>
  <Transition name="default-modal">
    <dialog ref="baseModalPopover" class="create-movie-form" :id="modal.id">
      <form  @submit.prevent id="create-movie-form">
        <h1>New Movie</h1>
        <div class="new-movie-form">
          <label> Name </label>
          <input id="movie-name" name="name" autofocus  :value="form.name" >
        </div>
        <div class="new-movie-form">
          <label>Description</label>
          <textarea
            :value="form.description"
            name="description"
          ></textarea>
        </div>
        <div class="new-movie-form">
          <label>Image</label>
          <input type="url" :value="form.image" name="image" >
        </div>
        <div class="new-movie-form">
          <label>Genres</label>
          <select
            :value="form.genres"
            id="genre-choice"
            name="genres"
            multiple
          >
            <option
              v-for="genre in genres"
              :key="genre.id"
              :selected="form.genres.includes(genre)"
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
            :value="form.inTheaters"
          />
        </div>
        <footer class="new-movie-form-footer">
          <button
            @click="closeModal(baseModalPopover)"
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

<script setup>
import { ref,reactive, onMounted, nextTick } from "vue";
import genres from "@/assets/data/genre-database.json";
import useModal from "@/stores/modal.js";
import mulligans from "@/mulligans-data.json";
import { closeModal, addMovie } from "@/helpers.js";

const modal = useModal();
const baseModalPopover = ref(null);
const form = reactive(modal.partialState)
const isOpen = ref()

const validationRules = (rule) => {
  if (rule === "required") return /^ *$/;
  return null;
};

const handleSubmit = async ({ target: { form } }) => {
  const payload = {...mulligans };
  const options = {
    mode: "cors",
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  };
  modal.state = [...modal.state, addMovie(payload)];
  try {
    const resp = await fetch(
      new URL("http://localhost:3000/movies/new"),
      options,
    );
    Object.assign(payload, resp);
    if (resp.ok) {
      Object.assign(payload, await resp.json());
      baseModalPopover.value.close("Movie added");
    }
  } catch ({ message, name }) {
    Object.assign(payload, { message, name });
  } finally {
    return payload;
  }
};

defineExpose({baseModalPopover});

</script>
