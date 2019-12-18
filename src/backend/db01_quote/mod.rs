#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::db01_quote;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "db01_quote"]
pub struct DB01Quote {
    pub id: i32,
    pub da01_user_id: i32,
    pub author: String,
    pub author_activity: String,
    pub quote: String,
    pub sw_published: bool,
}
