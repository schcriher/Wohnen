use diesel::result::Error;

use super::models::{House, NewHouse};
use super::repository::HouseRepository;

pub struct HouseService {
    pub repository: HouseRepository,
}

impl HouseService {
    pub fn new() -> Self {
        HouseService {
            repository: HouseRepository::new(),
        }
    }

    pub fn get_houses(&mut self) -> Result<Vec<House>, Error> {
        self.repository.find_all()
    }

    pub fn create_house(
        &mut self,
        kind: &str,
        street: &str,
        number: i32,
        floor: i32,
        postcode: &str,
        rooms: i32,
        baths: i32,
        area: f32,
    ) -> Result<bool, Error> {
        let new_house = NewHouse {
            kind: kind.to_string(),
            street: street.to_string(),
            number,
            floor,
            postcode: postcode.to_string(),
            rooms,
            baths,
            area,
        };
        let created = self.repository.create(&new_house)?;
        Ok(created)
    }
}
