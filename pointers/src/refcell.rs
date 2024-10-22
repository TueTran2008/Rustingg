use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
enum RefCellState {
    Unshared,
    Shared(i32),
    Exclusive,
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    reference: isize,
    state: crate::cell::Cell<RefCellState>,
}

impl<T> RefCell<T> {
    fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            reference: 0,
            state: Cell::new(RefCellState::Unshared),
        }
    }
    fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state = RefCellState::Shared(1);
                Some(unsafe { &*self.value.get() })
            }
            RefCellState::Shared(_) => Some(unsafe { &*self.value.get() }),
            RefCellState::Exclusive => None,
        }
    }
    fn borrow_mut(&self) -> Option<&mut T> {
        if let RefCellState::Unshared = self.state {
            self.state = RefCellState::Exclusive;
            Some(unsafe { &mut *self.value.get() })
        } else {
            None
        }
    }
}
