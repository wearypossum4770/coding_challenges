import { defineStore, acceptHMRUpdate } from "pinia";
import { unref, ref } from "vue";

const getters = {};

const actions = {};

const state = () => ({});

const store = defineStore("use-form", { getters, actions, state });
export default store;
if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(store, import.meta.hot));
}
