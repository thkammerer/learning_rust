// |...| {...}

// Tests --------------------------------------------------------------------------
mod test {
    //    use super::*;

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
            println!("The division result is {}", x / y);
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

    #[test]
    fn test_closure_diff_styles() {
        let define_inputs_and_outputs = |x: u32| -> u32 { x + 1 };
        let skip_inputs_and_outputs = |x| x + 1;
        let skip_bracket_if_single_statement = |x| x + 1;

        assert_eq!(define_inputs_and_outputs(3), 4);
        assert_eq!(skip_bracket_if_single_statement(3), 4);
        assert_eq!(skip_inputs_and_outputs(3), 4);
    }

    #[test]
    fn test_closure_with_para_immutable() {
        let mut vec_1 = vec![1, 2, 3];

        let some_closure = || {
            println!("Vec 1: {:?}", vec_1);
        };
        println!("Vec 1: {:?}", vec_1);

        // vec_1[1] = 666; // mutable borrow occurs here
        some_closure();
        vec_1[1] = 666;
    }

    #[test]
    fn test_closure_with_para_mutable() {
        let mut vec_1 = vec![1, 2, 3];

        let mut some_closure = || {
            println!("Vec 1: {:?}", vec_1);
            vec_1.push(35);
        };

        // vec_1[1] = 666; // mutable borrow occurs here
        some_closure();
        vec_1[1] = 666;
    }

    #[test]
    fn test_closure_with_para_move() {
        let mut vec_1 = vec![1, 2, 3];

        let mut some_closure = || {
            let vec2 = vec_1;
        };

        // vec_1[1] = 666; // mutable borrow occurs here
        some_closure();
        // println!( "Vec 1: {:?}", vec_1); // borrow of moved value
        // println!( "Vec 2: {:?}", vec_2);  // vec_2 not known here
    }
}
