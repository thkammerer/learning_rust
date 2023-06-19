fn dvision(divident: f64, divisor: f64) -> Result<f64, String> {
    if divisor.abs() < f64::EPSILON {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident / divisor)
    }

    // match divisor {
    //     0.0 => Err(String::from("Error: Division by zero")),
    //     _ => Ok(divident / divisor),
    // }
}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    #[test]
    fn test_simple_result_enum() {
        println!(
            "division of {} with {} results in {:?}",
            2.0,
            3.0,
            dvision(2.0, 3.0)
        );

        println!(
            "division of {} with {} results in {:?}",
            2.0,
            0.0,
            dvision(2.0, 0.0)
        );
    }

    #[test]
    fn test_result_enum() {
        let some_vec = vec![2, 5, 5, 6, 1, 9];
        let result = match some_vec.get(5) {
            Some(value) => Ok(value),
            None => Err("The value does not exist"),            
        };
        println!( "The value of result is {:?}", result );
    }
}
