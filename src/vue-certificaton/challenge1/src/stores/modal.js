import { defineStore, acceptHMRUpdate } from "pinia";
import { unref, ref } from "vue";

const getters = {};

const actions = {
  async showModalDecorator(wrapped) {
    return function decorator(...args) {
      const result = wrapped.call(this, args);
      return { ...args, result };
    };
  },

  /**
   * @description noop if modal is opened.
   * @returns { Promise.<this> } method is chainable.
   */
  async openModal() {
    if (!this.isOpen && !this.ariaModal) {
      this.isOpen = true;
    }
    return this;
  },
  /**
   *
   * @param { HTMLDialogElement } dialog
   * @returns
   */
  async showModal(dialog) {
    const modal = unref(dialog);
    modal.showModal();
    return this;
  },
  async show() {
    this.isOpen = true;
    return this;
  },
  /**
   * @description noop if modal is closed.
   * @returns { Promise.<this> } method is chainable.
   */
  async closeModal() {
    if (this.isOpen && !this.ariaModal) {
      this.isOpen = false;
    }
    return this;
  },
};
/**
 * @type ModalState
 * @param { boolean } isOpen - if modal should show.
 * @param { boolean } modalIsOpen - See {@link ModalState.isOpen}
 * @param { string } ariaLabelledby - the element id for that references aria-labelledby custom property for modal ** DO NOT USE ON DIALOG **
 * @param { string } ariaDescribedby - the element id for that references aria-describedby custom property for modal ** DO NOT USE ON DIALOG **
 * @returns { Object } an initial state object.
 */
const state = () => ({
  modalIsOpen: false,
  isOpen: false,
  id: "favorit-animal-modal",
  ariaLabelledby: "dialog1Title",
  ariaDescribedby: "dialog1Desc",
  ariaModal: false,
  baseModalPopover: ref(null),
});
/**
 * @type StoreDefinition
 * @name ModalStoreDefinition
 */
const store = defineStore("modal", { getters, actions, state });
export default store;
if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(store, import.meta.hot));
}
