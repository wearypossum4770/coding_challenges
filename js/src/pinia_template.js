import { defineStore } from 'pinia';

const getters = {
  chorus: ({ bottlesOfBeer }) => (bottlesOfBeer !== 1 ? `${bottlesOfBeer} bottles of beer on the wall, ${bottlesOfBeer} bottles of beer.
    Take one down and pass it around, ${bottlesOfBeer - 1} bottles of beer on the wall.` : `1 bottle of beer on the wall, 1 bottle of beer.
    Take one down and pass it around, no more bottles of beer on the wall.
    No more bottles of beer on the wall, no more bottles of beer.
    Go to the store and buy some more, 99 bottles of beer on the wall.`),
};
const actions = {
  async runner() {
    for (let beer = 0; beer < this.bottlesOfBeer; beer += 1) {
      console.log(this.chorus);
      this.takeBeer();
    }
    this.$reset();
  },
  async takeBeer() {
    this.bottlesOfBeer -= 1;
  },
};
const state = () => ({
  bottlesOfBeer: 99,
});

export default defineStore('pinia_template', {
  getters,
  actions,
  state,
});

export {
  getters as piniaTemplateGetters,
  actions as piniaTemplateActions,
  state as piniaTemplateState,
};
