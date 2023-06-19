enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
}

impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance: f32 = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
            Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
            Conveyance::Air(miles) => *miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;
//    use approx::assert_relative_eq;

    #[test]
    fn test_enum() {
        let train_user = Conveyance::Train(120);
//        println!("The value of the option 1 is {}", train_user as i32);

        let car_user = Conveyance::Car(60);
//        println!("The value of the option 2 is {}", car_user as i32);

        let plain_user = Conveyance::Air(60);
//        println!("The value of the option 3 is {}", plain_user as i32);


        println!( "the train user has a travel allowance of {}", train_user.travel_allowance());
        println!( "the car user has a travel allowance of {}", car_user.travel_allowance());
        println!( "the air user has a travel allowance of {}", plain_user.travel_allowance());
    }
}
