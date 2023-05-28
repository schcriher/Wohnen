use diesel::{AsChangeset, Insertable, Queryable};

use super::schema::houses;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = houses)]
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

#[derive(Debug, Insertable)]
#[diesel(table_name = houses)]
pub struct NewHouse {
    pub kind: String,
    pub street: String,
    pub number: i32,
    pub floor: i32,
    pub postcode: i32,
    pub rooms: i32,
    pub baths: i32,
    pub area: f32,
}
