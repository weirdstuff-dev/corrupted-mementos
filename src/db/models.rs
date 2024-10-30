use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::corrupted_mementos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Memento {
    pub extrinsic_hash: String,
    pub extrinsic_id: i32,
    pub block_id: i32,
    pub block_hash: String,
    pub minted: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::db::schema::corrupted_mementos)]
pub struct NewMemento {
    pub extrinsic_hash: String,
    pub extrinsic_id: i32,
    pub block_id: i32,
    pub block_hash: String,
    pub minted: i32,
}