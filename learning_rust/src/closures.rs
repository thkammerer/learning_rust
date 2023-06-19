// |...| {...}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    #[test]
    fn test_simple_closure() {
        let x = 5;
        let square = || println!("The square of the variable is {}", x * x);
        square();
    }

    #[test]
    fn test_simple_closure_with_inputs() {
        let x = 5;
        let square = |num: i32| println!("The square of the variable is {}", num * num);
        square(x);

        let y = 15;
        square(y);
    }

    #[test]
    fn test_simple_closure_with_some_inputs() {
        let print_info = |general_info: String, name: &str, age| {
            println!("{}\n\t {}: {}", general_info, name, age)
        };

        let general_info = String::from("The details are ");
        let (person_name, person_age) = (String::from("Thomas"), 55);

        print_info(general_info, &person_name, person_age);

        // println!("The variable has been moved {}", general_info);
        println!("The variable has not been moved {}", person_name);
        println!("The variable has not been moved {}", person_age);
    }

    fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
        if f(y) == true {
            println!("The division result is {}", x/y);
        } else {
            println!("The division is not possible");
        }
    }

    #[test]
    fn test_closure_as_parameter() {
        let division_status = |y: f32| {
            if y != 0.0 {
                true
            } else {
                false
            }
        };

        division(10.0, 4.0, division_status);
        division(23.5, 0.0, division_status);
    }
}
