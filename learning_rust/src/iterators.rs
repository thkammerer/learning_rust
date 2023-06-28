
// Tests --------------------------------------------------------------------------
mod test {
    //    use super::*;

    #[test]
    fn test_using_iterators_with_closures() {
        let a = vec![1, -2, 4, 7, 666];

        let mut check = a.iter().any(|&x| x > 0);
        println!("The value of the any function is {}", check);

        let mut check = a.iter().all(|&x| x > 0);
        println!("The value of the all function is {}", check);

        let mut check = a.iter().find(|&&x| x > 6);
        println!("The value of the find function is {}", check.unwrap());

        let mut check = a.iter().find(|&&x| x < 0);
        println!("The value of the find function is {}", check.unwrap());

        let mut check = a.iter().position(|&x| x > 6);
        println!("The index of the position function is {}", check.unwrap());
        println!("The value of the index is {}", a[check.unwrap()]);

        println!("The maximum value ind vec a = {:?}", a.iter().max());
        println!("The minimum value ind vec a = {:?}", a.iter().min());

        println!("The reverse_iterator of vec a is = {:?}", a.iter().rev());
    }

    #[test]
    fn test_collect_func() {
        let my_vec = vec![1, 2, 3, 4, 5, 6, 7];
        let filter_result = my_vec.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
        println!("result of the filter function ist {:?}", filter_result);

        let my_vec_cpy = my_vec.clone();
        let filter_result = my_vec.into_iter().filter(|&x| x >= 5).collect::<Vec<u32>>();
        println!("result of the filter function ist {:?}", filter_result);

        let mut mapped_values = my_vec_cpy.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
        println!("result of mapped_values = {:?}", mapped_values);

        let mut mapped_values = my_vec_cpy
            .iter()
            .map(|x| 2 * *x)
            .filter(|&x| x >= 5)
            .collect::<Vec<u32>>();
        println!("result of mapped_values with filter = {:?}", mapped_values);
    }
}
