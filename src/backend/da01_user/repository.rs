#![allow(proc_macro_derive_resolution_fallback)]
use crate::backend::DA01User;
use crate::schema::da01_user;
use diesel;
use diesel::prelude::*;
pub fn all(connection: &PgConnection) -> QueryResult<Vec<DA01User>> {
    da01_user::table.load::<DA01User>(&*connection)
}

/*
pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Person> {
    people::table.find(id).get_result::<Person>(connection)
}
*/
pub fn insert(da01: DA01User, connection: &PgConnection) -> QueryResult<DA01User> {
    diesel::insert_into(da01_user::table)
        .values(&InsertableDA01User::from_da01(da01))
        .get_result(connection)
}
/*
pub fn update(id: i32, person: Person, connection: &PgConnection) -> QueryResult<Person> {
    diesel::update(people::table.find(id))
        .set(&person)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(connection)
}
*/
#[derive(Insertable)]
#[table_name = "da01_user"]
struct InsertableDA01User {
    name: String,
    mail: String,
    password: String,
    active: bool,
}

impl InsertableDA01User {
    fn from_da01(da01: DA01User) -> InsertableDA01User {
        InsertableDA01User {
            name: da01.name,
            mail: da01.mail,
            password: da01.password, // to do better
            active: da01.active,
        }
    }
}
