import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import "./main.css";
// import "../style.css";

const pinia = createPinia();

const app = createApp(App);

app.use(pinia).mount("#app");
