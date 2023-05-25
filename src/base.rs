pub struct Error;

pub const HOUSE_TYPES: &[&str] = &["Casa", "Loft", "Chalet", "Dúplex", "Apartamento"];

pub struct House {
    pub id: i32,
    pub kind: String,
    pub street: String,
    pub number: i32,
    pub floor: i32,
    pub postcode: String,
    pub rooms: i32,
    pub baths: i32,
    pub area: f32,
}

pub trait DAO {
    fn get_houses(&mut self) -> Result<Vec<House>, Error>;
    fn create_house(&mut self, house: &House) -> Result<House, Error>;
    fn update_house(&mut self, house: &House) -> Result<bool, Error>;
    fn delete_house(&mut self, id: i32) -> Result<bool, Error>;
}
