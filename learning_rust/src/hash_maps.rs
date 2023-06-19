use std::collections::HashMap;

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    #[test]
    fn test_hash_map() {
        let mut body_heights: HashMap<&str, i32> = HashMap::new();
        body_heights.insert("Anatoli", 172);
        body_heights.insert("Garry", 184);
        body_heights.insert("Jan", 179);

        let key = "Garry";
        println!("{} is {} tall", key, body_heights.get(key).unwrap());

        let key = "Jan";
        if body_heights.contains_key(key) {
            println!( "The value for {} exists", key);
        }
        
        match body_heights.get(key) {
            Some(height) => println!( "The value exists, height =  {} cm", height),
            None => println!("The value does not exist"),
        }

        for( name, age ) in &body_heights {
            println!( "{} is {} cm tall", name, age );
        }
    }

    #[test]
    fn test_hash_map_one_value() {
        let mut likes: HashMap<&str, &str> = HashMap::new();
        likes.insert("Thomas", "playing chess");
        likes.insert("Thomas", "playing cards");
        println!( "Thomas likes {:?}", likes );
        
        likes.entry("Thomas").or_insert("jogging");
        println!( "Thomas likes {:?}", likes );
    }

    #[test]
    fn test_hash_map_from_vector() {
        let some_vec = vec![ 5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 666 ];
        let mut freq_vec: HashMap<i32, u32> = HashMap::new();

        for value in &some_vec {
            let freq =  freq_vec.entry(*value).or_insert(0);
            *freq += 1;
        }
        println!( "some_vec: {:?}", some_vec);
        println!( "freq_vec: {:?}", freq_vec);
    }
}
