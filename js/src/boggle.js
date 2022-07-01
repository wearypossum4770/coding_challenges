const scoreWord = (word) => {
  switch (word.length) {
    case 0:
      return 0;
    case 3:
    case 4:
      return 1;
    case 5:
      return 2;
    case 6:
      return 3;
    case 7:
      return 5;
    default:
      return 11;
  }
};
const startGame = () => null;
const endGame = () => null;
class Player {
  constructor(number) {
    this.score = 0;
    this.number;
  }
}
const myGameArea = () => null;
const board = () => {};
