use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

//unsafe implt <T> Sync for Cell<T> {}

//}
impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }
    fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
    fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
#[cfg(test)]
mod test {
    use super::Cell;
    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = std::sync::Arc::new(Cell::new(12));
        let x1 = Arc::clone(&x);
        std::thread::spawn(move || {
            x1.set(43);
        });
        let x2 = Arc::clone(&x);
        std::thread::spawn(move || {
            x2.set(53);
        });
    }
    // #[test]
    // fn bad2() {
    //     let x = Cell::new(vec![43]);
    //     let value = x.get();
    // }
}
