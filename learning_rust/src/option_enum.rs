struct Person {
    name: String,
    age: i32,
}

fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}

fn print_number(num: Option<i32>) {
    match num {
        Some(number) => println!("The number is {}", num.unwrap()),
        None => println!("We do not have any value"),
    };
}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    #[test]
    fn test_simple_option_enum() {
        let mut disease: Option<String> = None;
        disease = Some(String::from("Diabetes"));

        match disease {
            Some(disease_name) => println!("You have the disease of {}", disease_name),
            None => println!("You do not have any disease"),
        }
    }
    #[test]
    fn test_option_enum() {
        let s1: Option<&str> = Some("Some String");
        println!(
            "The value of s1 is {:?}, and the value of s1 itself after unwrapping is {:?}",
            s1,
            s1.unwrap()
        );

        let f1: Option<f64> = Some(10.54);
        let mut f2: f64 = 16.5;
        f2 += f1.unwrap();
        println!("The value of the sum is {}", f2);

        let v1 = Some(vec![0, 1, 2, 3]);

        let p1 = Person {
            name: String::from("Thomas"),
            age: 55,
        };

        let someone: Option<Person> = Some(p1);

        let some_number = Some(6);
        print_number(square(some_number));
        print_number(square(None));
    }
}
