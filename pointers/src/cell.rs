use std::cell::UnsafeCell;
#[warn(dead_code)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

//unsafe implt <T> Sync for Cell<T> {}
//unsafe impl<T> Sync for Cell<T> {}
//}
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
