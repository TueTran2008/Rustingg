mod cell;
mod refcell;

#[cfg(test)]
mod test {
    use super::Cell;
    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(12));
        let x1 = x.clone();
        std::thread::spawn(move || {
            x1.set(43);
        });
        let x2 = x.clone();
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
