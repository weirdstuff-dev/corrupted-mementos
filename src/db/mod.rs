pub(crate) mod schema;
pub(crate) mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::db::models::{Memento, NewMemento};
use crate::db::schema::corrupted_mementos;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn record_memento(new_memento: &NewMemento) {
    let mut conn = establish_connection();

    diesel::insert_into(corrupted_mementos::table)
        .values(new_memento)
        .returning(Memento::as_returning())
        .get_result::<Memento>(&mut conn).expect("unable to insert");
}

// TODO optimize later
pub fn get_all_mementos() {
    let mut conn = establish_connection();

    let mementos = corrupted_mementos::table.load::<Memento>(&mut conn).unwrap();

    println!("{:?}", mementos)
}