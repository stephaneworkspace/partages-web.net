//#![allow(proc_macro_derive_resolution_fallback)]
// use super::schema::da01_user;

pub mod handler;
pub mod repository;
pub mod router;

// #[derive(Queryable, AsChangeset, Serialize, Deserialize)]
// #[table_name = "da01_user"]
pub struct DA01User {
    pub id: i32,
    pub name: String,
    pub mail: String,
    pub password: i32, // encrypt later
    pub active: bool,
}
