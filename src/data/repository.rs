use diesel::{prelude::*, result::Error, SqliteConnection};

use dotenvy::dotenv;
use std::env;

use super::models::{House, NewHouse};
use super::schema::houses::dsl::*;

pub struct HouseRepository {
    pub conn: SqliteConnection,
}

impl HouseRepository {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        HouseRepository {
            conn: SqliteConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }

    pub fn find_all(&mut self) -> Result<Vec<House>, Error> {
        houses.load::<House>(&mut self.conn)
    }

    pub fn create(&mut self, new_house: &NewHouse) -> Result<bool, Error> {
        let rows = diesel::insert_into(houses)
            .values(new_house)
            .execute(&mut self.conn)?;
        println!("{:?}", rows); //////////////////////////////////////////////////////////////////
        Ok(rows > 0)
    }

    pub fn update(&mut self, house: House) -> Result<bool, Error> {
        let rows = diesel::update(houses.find(house.id))
            .set(house)
            .execute(&mut self.conn)?;
        println!("{:?}", rows); //////////////////////////////////////////////////////////////////
        Ok(rows > 0)
    }

    pub fn delete(&mut self, h_id: i32) -> Result<bool, Error> {
        let rows = diesel::delete(houses.find(h_id)).execute(&mut self.conn)?;
        println!("{:?}", rows); //////////////////////////////////////////////////////////////////
        Ok(rows > 0)
    }
}
