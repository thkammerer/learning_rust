// Box Smart Pointer
//   usually every thing is allocated on the stack
//   but Box means allocating heap memory

// Tests --------------------------------------------------------------------------

mod test {
    //    use super::*;

    #[test]
    fn intro_box_smart_pointer() {
        let single_value = Box::new(0.625); // single value is on the stack, but 0.625: f64 is on the heap
        let x = 0.625; // here everthing is on the stack
        println!("Are the values being equal? Answer: {}", x == *single_value);

        let mut stack_var = 4;
        let stack_ref = &stack_var;
        let heap_var = Box::new(stack_var); // a copy the value of stack_var is allocated on the heap.

        stack_var = 5;
        println!("stack_var = {}, heap_var = {}", stack_var, heap_var);
    }

    // List is representing a simple linked list
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[test]
    fn using_box_smart_pointer_with_linked_list() {
        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("list = {:?}", list);
    }

    #[derive(Debug)]
    enum BetterList {
        Cons(i32, Option<Box<BetterList>>),
    }

    #[test]
    fn using_box_smart_pointer() {
        use BetterList::Cons;

        let list = Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));
        println!("BetterList = {:?}", list);
    }

    struct MySmartPointer {
        value: i32,
    }

    impl MySmartPointer {
        fn new(x: i32) -> Self {
            MySmartPointer { value: x }
        }
    }

    impl std::ops::Deref for MySmartPointer {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl std::ops::Drop for MySmartPointer {
        fn drop(&mut self) {
            println!(
                "dropping MySmartPointer object from memory {:?}",
                self.value
            );
        }
    }

    #[test]
    fn using_custom_defined_smart_pointer() {
        let a = 50;
        let b = Box::new(a);
        println!("is a equals 50? {}", 50 == a);
        println!("is *b eqauls 50? {}", 50 == *b);
        println!("is a == *b? {}", a == *b);

        let my_smart_pointer_1 = MySmartPointer::new(a);
        let my_smart_pointer_2 = MySmartPointer::new(*b + 10);
        println!(
            "is a equal to my_smart_pointer_1? {}",
            a == *my_smart_pointer_1 // calls the Deref trait
        );
    }
}
