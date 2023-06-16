pub fn simple_loop(cycles: i32) -> i32 {
    println!("simple_loop");
    let mut i = cycles;
    loop {
        println!("i = {}", i);
        i -= 1;
        if 0 == i {
            break i;
        }
    }
}

pub fn while_loop(cycles: i32) -> i32 {
    println!("´while_loop");
    let mut i = cycles;
    while i > 0 {
        println!("i = {}", i);
        i -= 1;
    }
    i
}

pub fn simple_for_loop(cycles: i32) -> i32 {
    println!("simple_for_loop");
    for i in 1..=cycles {
        println!("i = {}", i);
    }
    cycles
}

pub fn iterate_vector(vector: Vec<i32>) -> usize {
    let mut index = 0;
    for value in vector.iter() {
        println!("value[{}] of vector is {}", index, value);
        index += 1;
    }
    vector.len()
}

pub fn simple_match(roman_number: char) -> i32 {
    let mut value = 0;
    match roman_number {
        'i' | 'I' => value = 1,
        'v' | 'V' => value = 5,
        'x' | 'X' => value = 10,
        'l' | 'L' => value = 50,
        'c' | 'C' => value = 100,
        'd' | 'D' => value = 500,
        'm' | 'M' => value = 1000,
        _ => println!("keine römische Ziffer"),
    }
    value
}

// --- Tests --------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple_loop() {
        assert_eq!(simple_loop(5), 0);
    }

    #[test]
    fn test_while_loop() {
        assert_eq!(while_loop(10), 0);
    }

    #[test]
    fn test_simple_for() {
        assert_eq!(simple_for_loop(6), 6);
    }

    #[test]
    fn test_iterator_vector() {
        let values = vec![17, 4, 32, 666];
        assert_eq!(iterate_vector(values), 4);
    }

    #[test]
    fn test_simple_matcher() {
        assert_eq!(simple_match('I'), 1 );
        assert_eq!(simple_match('V'), 5 );
        assert_eq!(simple_match('X'), 10 );        
        assert_eq!(simple_match('L'), 50 );
        assert_eq!(simple_match('C'), 100 );
        assert_eq!(simple_match('D'), 500 );        
        assert_eq!(simple_match('M'), 1000 );        

        assert_eq!(simple_match('i'), 1 );
        assert_eq!(simple_match('v'), 5 );
        assert_eq!(simple_match('x'), 10 );        
        assert_eq!(simple_match('l'), 50 );
        assert_eq!(simple_match('c'), 100 );
        assert_eq!(simple_match('d'), 500 );        
        assert_eq!(simple_match('m'), 1000 );        

    }
}
