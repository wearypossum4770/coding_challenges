export const showModal = ($refs) => {
  console.log($refs);
  $refs.baseModalPopover.showModal();
};

export const displayProps = ({ rating }) => (rating > 0 ? rating : "-");

export const handleRating = (movieId, star, state) =>
  (state[movieId - 1].rating = star);
