use core::num;
use gcd::Gcd;
use std::cmp;
use std::ops;

pub struct Rational {
    numerator: i32,
    denominator: i32,
}

impl Rational {
    pub fn new_from_integer(integer: i32) -> Self {
        Self {
            numerator: integer,
            denominator: 1,
        }
    }

    pub fn new(numerator: i32, denominator: i32) -> Self {
        create_reduced_radional(numerator, denominator)
    }

    pub fn as_floating_point(&self) -> f32 {
        let result: f32 = self.numerator as f32 / self.denominator as f32;
        result
    }
}

impl ops::Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        let mut result = Rational::new(self.numerator, self.denominator);
        result += rhs;
        result
    }
}

impl ops::AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        let new_numerator = self.denominator * rhs.numerator + self.numerator * rhs.denominator;
        let new_denominator = self.denominator * rhs.denominator;
        *self = create_reduced_radional(new_numerator, new_denominator);
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        let mut result = Rational::new(self.numerator, self.denominator);
        result -= rhs;
        result
    }
}

impl ops::SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Rational) {
        let new_numerator = self.numerator * rhs.denominator - self.denominator * rhs.numerator;
        let new_denominator = self.denominator * rhs.denominator;
        *self = create_reduced_radional(new_numerator, new_denominator);
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        let mut result = Rational::new(self.numerator, self.denominator);
        result *= rhs;
        result
    }
}

impl ops::MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        let new_numerator = self.numerator * rhs.numerator;
        let new_denominator = self.denominator * rhs.denominator;
        *self = create_reduced_radional(new_numerator, new_denominator);
    }
}

impl ops::Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        let mut result = Rational::new(self.numerator, self.denominator);
        result /= rhs;
        result
    }
}

impl ops::DivAssign<Rational> for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        let new_numerator = self.numerator * rhs.denominator;
        let new_denominator = self.denominator * rhs.numerator;
        *self = create_reduced_radional(new_numerator, new_denominator);
    }
}

impl cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

fn create_reduced_radional(mut numerator: i32, mut denominator: i32) -> Rational {
    if denominator == 0 {
        panic!("The denominator of a rational number must not be 0!")
    }
    if denominator < 0 {
        numerator *= -1;
        denominator *= -1;
    }
    let common_divisor = Gcd::gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i32;
    Rational {
        numerator: numerator / common_divisor,
        denominator: denominator / common_divisor,
    }
}

// --- Tests --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_rational() {
        let rational = Rational::new(1, 2);
        assert_eq!(rational.numerator, 1);
        assert_eq!(rational.denominator, 2);

        let rational_neg_numerator = Rational::new( -1, 3 );
        assert_eq!(rational_neg_numerator.numerator, -1);
        assert_eq!(rational_neg_numerator.denominator, 3);

        let rational_neg_denominator = Rational::new( 2, -3 );
        assert_eq!(rational_neg_denominator.numerator, -2);
        assert_eq!(rational_neg_denominator.denominator, 3);

        let rational_neg_numerator_and_denominator = Rational::new( -2, -5 );
        assert_eq!(rational_neg_numerator_and_denominator.numerator, 2);
        assert_eq!(rational_neg_numerator_and_denominator.denominator, 5);
    }
    
    #[test]
    #[should_panic]
    fn try_to_create_a_rational_denominator_zero() {
        let rational = Rational::new( 2, 0 );
    }

    #[test]
    fn create_rational_from_integer() {
        let rational = Rational::new_from_integer(3);
        assert_eq!(rational.numerator, 3);
        assert_eq!(rational.denominator, 1);
    }

    #[test]
    fn create_rational_and_reduce() {
        let mut rational = Rational::new(8, 12);
        assert_eq!(rational.numerator, 2);
        assert_eq!(rational.denominator, 3);
    }

    #[test]
    fn adding_two_rationals() {
        let a_quarter = Rational::new(1, 4);
        let a_half = Rational::new(1, 2);
        let three_quarter = Rational::new(3, 4);
        let result = a_quarter + a_half;
        assert_eq!(result.numerator, 3);
        assert_eq!(result.denominator, 4);
        assert!(result == three_quarter);
    }

    #[test]
    fn subtracting_two_rationals() {
        let a_quarter = Rational::new(1, 4);
        let one = Rational::new_from_integer(1);
        let three_quarter = Rational::new(3, 4);
        let result = one - a_quarter;
        assert_eq!(result.numerator, 3);
        assert_eq!(result.denominator, 4);
        assert!(result == three_quarter);
    }

    #[test]
    fn multiplicating_two_rationals() {
        let a_half = Rational::new(1, 2);
        let three_quarter = Rational::new(3, 4);
        let result = a_half * three_quarter;
        assert!(result == Rational::new(3, 8));
    }

    #[test]
    fn divide_two_rationals() {
        let a_half = Rational::new(1, 2);
        let three_quarter = Rational::new(3, 4);
        let result = a_half / three_quarter;
        assert!(result == Rational::new(2, 3));
    }

    #[test]
    fn as_floating_point() {
        let test = Rational::new(1, 8);
        assert_eq!(test.as_floating_point(), 0.125);
    }
}
