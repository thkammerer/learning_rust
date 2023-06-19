
// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
        first_str
        //second_str
    }
    
    #[test]
    fn problematic_lifetime() {
        let s_1 = "Hello";
        let v: &str;
        {
            let s_2 = String::from("World");
            v = some_fn(s_1, s_2.as_str());            
        }
        println!("{}", v);

    }

    fn greater(a: &i32, b: &i32) -> i32 {
        if a > b {
            *a
        }
        else {
            *b
        }
    }
    
    #[test]
    fn unproblematic_no_reference() {
        let value_1 = 5;
        let value_2 = 10;
        let result = greater( &value_1, &value_2);
        println!( "The greater value of {} and {} is {}", value_1, value_2, result);
    }


    fn ref_greater<'a, 'b> ( x: &'a i32, y: &'a i32 ) -> &'a i32 {
        if x > y {
            x
        } else {
            y
        }
    }
    
    #[test]
    fn problemetic_references() {
        let value_1 = 5;
        {
            let value_2 = 10;
            let result = ref_greater( &value_1, &value_2);
            println!( "The larger value is {}", result);   
        }
    }

    struct Person <'a> {
        name: &'a str,
        age: i32,
    }

    #[test]
    fn problemetic_struct() {
        let first_name = "Thomas";
        let mut thomas = Person {
            name: &first_name,
            age: 55,
        };

        {
            let last_name = String::from("Philipp");
            thomas.name = &last_name;
        }
        // Fehler: println!( "The name of the person is {} and his age is {}", thomas.name, thomas.age );
    }
    
    fn use_vec<'a> (vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
        if 3 > 5 {
            vec1
        } else {
            vec2
        }
    }

    #[test]
    fn problematic_references_same_val() {
        let some_vec = vec![5, 8, 8, 7, 5, 2];
        let return_vec = use_vec(&some_vec, &some_vec);
        println!("the return vector is {:?}", return_vec);
    }

}
