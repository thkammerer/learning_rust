// use std::ops;

fn square<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U>  Point<T, U> 
where T: std::fmt::Debug, U: std::fmt::Debug {
    fn print(&self) {
        println!( "The value of the point coordinates are {:?} , {:?}", self.x , self.y);
    }
}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_square() {
        assert_eq!(square(5), 25);
        assert_relative_eq!(square(6.0), 36.0, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_point() {
        let p1: Point<i32, i32> = Point { x: 5, y: 5 };
        let p2: Point<f32, f32> = Point { x: 5.0, y: 6.2 };
        let p3: Point<i32, f64> = Point { x: 5, y: 5.0 };

        p1.print();
        p2.print();
        p3.print();
    }
}
