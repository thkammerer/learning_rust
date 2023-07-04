use std::rc::Rc;

enum List {
    //    Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

// --- Tests --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_list() {
        let a = Rc::new(List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Nil)))));
        println!("Counting after creating a = {}", Rc::strong_count(&a));

        let b = Rc::new(List::Cons(3, Rc::clone(&a)));
        println!("Counting after creating b = {}", Rc::strong_count(&a));

        let c = Rc::new(List::Cons(4, Rc::clone(&a)));
        println!("Counting after creating c = {}", Rc::strong_count(&a));
    }
}
