import { describe, test, expect, beforeEach} from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import useModal from '@/stores/modal.js'



describe('modal state management', () => {
    beforeEach(() => {
        // creates a fresh pinia and make it active so it's automatically picked
        // up by any useStore() call without having to pass it to it:
        // `useStore(pinia)`
        setActivePinia(createPinia())
      })
    test('when close modal function is called  ')
    test('when open modal function is called isOpen is true', async () => {
        const modal = useModal()
        expect(modal.isOpen).toBe(false)
        await modal.openModal()
        expect(modal.isOpen).toBe(true)
    })
    test('when open modal function is called and modal is open, the isOpen value does not change.', async () => {
        const modal = useModal()
        modal.isOpen = true
        expect(modal.isOpen).toBe(true)
        await modal.openModal()
        expect(modal.isOpen).toBe(true)
    })
});

