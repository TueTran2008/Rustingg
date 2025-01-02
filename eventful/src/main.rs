enum List {
    Con(i32, Rc<List>),
    Nil,
}
use crate::List::*;
use std::rc::*;
fn main() {
    let a = Rc::new(Con(1, Rc::new(Con(10, Rc::new(Nil)))));
    let b = Con(2, a.clone());
    let c = Con(3, a.clone());
    println!("{}", Rc::strong_count(&a));
}
