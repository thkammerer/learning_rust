struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl Data {
    fn new(data: Vec<i32>) -> Self {
        Self { some_data: data }
    }
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum: i32 = 0;
        for cur_value in self.some_data.iter() {
            sum += *cur_value;
        }
        sum as f32 / self.some_data.len() as f32            
    }

    fn variance(&self) -> f32 {
        let mean = self.mean();
        let mut sum_squared_diff = 0.0;
        for cur_value in self.some_data.iter() {
            sum_squared_diff += (*cur_value as f32 - mean) * (*cur_value as f32 - mean);
        }
        sum_squared_diff / self.some_data.len() as f32
    }
}

// Tests --------------------------------------------------------------------------
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_mean() {
        let my_data = Data::new(vec![1, 4, 7, 11, 17]);
        assert_relative_eq!(my_data.mean(), 8.0, epsilon = f32::EPSILON);
    }

    #[test]
    fn test_variance() {
        let my_data = Data::new(vec![1, 4, 7, 11, 17]);        
        assert_relative_eq!(my_data.variance(), 31.2, epsilon = f32::EPSILON);
    }
}
