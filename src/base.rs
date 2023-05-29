use std::{fmt::Display, str::FromStr};

use strsim::jaro_winkler;

pub struct Error;

pub const HOUSE_TYPES: &[&str] = &["Casa", "Loft", "Chalet", "Dúplex", "Apartamento"];

#[derive(Debug, Default)]
pub struct House {
    pub id: i32,
    pub kind: String,
    pub street: String,
    pub number: i32,
    pub floor: i32,
    pub postcode: i32,
    pub rooms: i32,
    pub baths: i32,
    pub area: f32,
}

pub trait Apply {
    fn apply<U: Display>(&self, value: &U) -> bool;
}

#[derive(Debug, Default)]
pub struct Text {
    pub text: String,
}

impl Apply for Text {
    fn apply<U: Display>(&self, value: &U) -> bool {
        self.text == "" || jaro_winkler(&self.text, &value.to_string()) > 0.69
    }
}

#[derive(Debug)]
pub struct Range<T: PartialOrd + FromStr> {
    pub min: T,
    pub max: T,
}

macro_rules! range_default_impl {
    ($t:ty, $max:expr) => {
        impl Default for Range<$t> {
            fn default() -> Self {
                Range {
                    min: Default::default(),
                    max: $max,
                }
            }
        }
    };
}
range_default_impl!(i32, i32::MAX);
range_default_impl!(f32, f32::MAX);

impl<T: PartialOrd + FromStr> Apply for Range<T> {
    fn apply<U: Display>(&self, value: &U) -> bool {
        match value.to_string().parse::<T>() {
            Ok(value) => value >= self.min && value <= self.max,
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Filter {
    pub kind: Text,
    pub street: Text,
    pub number: Range<i32>,
    pub floor: Range<i32>,
    pub postcode: Range<i32>,
    pub rooms: Range<i32>,
    pub baths: Range<i32>,
    pub area: Range<f32>,
}

impl Filter {
    pub fn valid(&self, house: &House) -> bool {
        self.kind.apply(&house.kind)
            && self.street.apply(&house.street)
            && self.number.apply(&house.number)
            && self.floor.apply(&house.floor)
            && self.postcode.apply(&house.postcode)
            && self.rooms.apply(&house.rooms)
            && self.baths.apply(&house.baths)
            && self.area.apply(&house.area)
    }
}

pub trait DAO {
    fn get_houses(&mut self) -> Result<Vec<House>, Error>;
    fn create_house(&mut self, house: &House) -> Result<House, Error>;
    fn update_house(&mut self, house: &House) -> Result<bool, Error>;
    fn delete_house(&mut self, id: i32) -> Result<bool, Error>;
}
