use std::{fmt::Debug, str::FromStr};

pub fn test_positive_and_zero_number<T>(value: &str) -> bool
where
    T: PartialOrd<T> + FromStr,
    <T as FromStr>::Err: Debug,
{
    if let Ok(n) = value.to_string().trim().parse::<T>() {
        return n >= "0".to_owned().parse::<T>().unwrap();
    }
    false
}
