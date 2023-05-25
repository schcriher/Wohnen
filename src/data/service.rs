use super::models::{House as DbHouse, NewHouse as DbNewHouse};
use super::repository::Repository;

use crate::base::{Error, House, DAO};

pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new() -> Self {
        Service {
            repository: Repository::new(),
        }
    }
}

impl DAO for Service {
    fn get_houses(&mut self) -> Result<Vec<House>, Error> {
        let houses = self.repository.find_all();
        match houses {
            Ok(houses) => {
                let houses: Vec<House> = convert_vector(houses);
                Ok(houses)
            }
            Err(_) => Err(Error),
        }
    }

    fn create_house(&mut self, house: &House) -> Result<House, Error> {
        let house: DbNewHouse = house.into();
        let house = self.repository.create(&house);
        match house {
            Ok(house) => {
                let house: House = house.into();
                Ok(house)
            }
            Err(_) => Err(Error),
        }
    }

    fn update_house(&mut self, house: &House) -> Result<bool, Error> {
        let house: DbHouse = house.into();
        self.repository.update(&house).map_err(|_| Error)
    }

    fn delete_house(&mut self, id: i32) -> Result<bool, Error> {
        self.repository.delete(id).map_err(|_| Error)
    }
}

impl From<&House> for DbHouse {
    fn from(house: &House) -> Self {
        DbHouse {
            id: house.id,
            kind: house.kind.clone(),
            street: house.street.clone(),
            number: house.number,
            floor: house.floor,
            postcode: house.postcode.clone(),
            rooms: house.rooms,
            baths: house.baths,
            area: house.area,
        }
    }
}

impl From<&House> for DbNewHouse {
    fn from(house: &House) -> Self {
        DbNewHouse {
            kind: house.kind.clone(),
            street: house.street.clone(),
            number: house.number,
            floor: house.floor,
            postcode: house.postcode.clone(),
            rooms: house.rooms,
            baths: house.baths,
            area: house.area,
        }
    }
}

impl From<DbHouse> for House {
    fn from(house: DbHouse) -> Self {
        House {
            id: house.id,
            kind: house.kind,
            street: house.street,
            number: house.number,
            floor: house.floor,
            postcode: house.postcode,
            rooms: house.rooms,
            baths: house.baths,
            area: house.area,
        }
    }
}

impl From<DbNewHouse> for House {
    fn from(house: DbNewHouse) -> Self {
        House {
            id: -1,
            kind: house.kind,
            street: house.street,
            number: house.number,
            floor: house.floor,
            postcode: house.postcode,
            rooms: house.rooms,
            baths: house.baths,
            area: house.area,
        }
    }
}

fn convert_vector<T, U>(vector: Vec<T>) -> Vec<U>
where
    U: From<T>,
{
    vector.into_iter().map(U::from).collect()
}
