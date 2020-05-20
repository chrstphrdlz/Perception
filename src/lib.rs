fn squared_deviation_from_the_mean(x: &Vec<f64>) -> f64 {
    let avg = average(&x);
    return x
        .into_iter()
        .map(|val| val - avg)
        .map(|difference_from_mean| difference_from_mean * difference_from_mean)
        .sum();
}
pub fn sample_variance(x: &Vec<f64>) -> f64 {
    let sample_size = x.len();
    if sample_size <= 1 {
        panic!("Need at least two values to calculate the variance of a sample");
    }

    return squared_deviation_from_the_mean(&x) / (sample_size - 1) as f64;
}

pub fn sample_std_dev(x: &Vec<f64>) -> f64 {
    return sample_variance(x).sqrt();
}
pub fn average(x: &Vec<f64>) -> f64 {
    return x.into_iter().sum::<f64>() / (x.len() as f64);
}

#[cfg(test)]
mod tests {
    use crate::{average, sample_std_dev, sample_variance};

    #[test]
    #[should_panic]
    fn test_sample_variance_zero_length_vector() {
        let vec: Vec<f64> = Vec::new();
        sample_variance(&vec);
    }

    #[test]
    #[should_panic]
    fn test_standard_deviation_singe_item_vector() {
        let mut vec = Vec::new();
        vec.push(1.0);

        sample_variance(&vec);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&vec![1.0, 2.0]), 1.5);
    }

    #[test]
    fn test_standard_deviation() {
        assert_eq!(
            sample_std_dev(&vec![1.0, 2.0, 3.0, 4.0]),
            1.2909944487358056
        );
    }

    #[test]
    fn test_variance() {
        assert_eq!(
            sample_variance(&vec![1.0, 2.0, 3.0, 4.0]),
            1.6666666666666667
        );
    }
}
