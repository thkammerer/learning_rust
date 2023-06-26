// Tests --------------------------------------------------------------------------
mod test {
    //    use super::*;

    fn max(x: i32, y: i32) -> i32 {
        if x > y {
            x
        } else {
            y
        }
    }

    fn min(x: i32, y: i32) -> i32 {
        if x < y {
            x
        } else {
            y
        }
    }

    #[test]
    fn test_simple() {
        let mut f1 = min;
        println!("The minimum of the values ia {}", f1(2, 3));
        let f2 = max;
        println!("The maximum of the values ia {}", f2(2, 3));
    }
}
