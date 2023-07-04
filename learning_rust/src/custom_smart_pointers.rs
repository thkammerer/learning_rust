struct MySmartPointer<T: std::fmt::Debug> {
    value: T,
}

impl<T: std::fmt::Debug> MySmartPointer<T> {
    fn new(x: T) -> Self {
        MySmartPointer { value: x }
    }
}

use std::ops::Deref;

impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: std::fmt::Debug> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!(
            "Dropping MySmartPointer<T> object from memory {:?}",
            self.value
        );
    }
}

// --- Tests --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_smart_pointer() {
        let sm = MySmartPointer::new(32);
        assert_eq!(sm.value, 32);
    }
}

fn my_fn(str: &str) {
    println!("The string received from the test_coercion is {}", str);
}

#[test]
fn test_coercion() {
    let sptr_1 = MySmartPointer::new("Thomas Kammerer");
    my_fn(&sptr_1); // deref coercion!!!  &MySmartPointer -> &String -> &str

    let some_vec = MySmartPointer::new(vec![1, 2, 3]);
    for z in &*some_vec {
        println!("The value is {}", z);
    }
}
