use crate::backend::repository_da01;
use crate::backend::DA01User;
use crate::connection::DbConn;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use std::env;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<DA01User>>, Status> {
    repository_da01::all(&connection)
        .map(|data| Json(data))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/hello-world")]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

/*
#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Person>, Status> {
    people::repository::get(id, &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}
*/
#[post("/", format = "application/json", data = "<da01>")]
pub fn post(
    da01: Json<DA01User>,
    connection: DbConn,
) -> Result<status::Created<Json<DA01User>>, Status> {
    repository_da01::insert(da01.into_inner(), &connection)
        .map(|da01| da01_created(da01))
        .map_err(|error| error_status(error))
}

fn da01_created(da01: DA01User) -> status::Created<Json<DA01User>> {
    status::Created(
        format!(
            "{host}:{port}/people/{id}",
            host = host(),
            port = port(),
            id = da01.id
        )
        .to_string(),
        Some(Json(da01)),
    )
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}
/*
#[put("/<id>", format = "application/json", data = "<person>")]
pub fn put(id: i32, person: Json<Person>, connection: DbConn) -> Result<Json<Person>, Status> {
    people::repository::update(id, person.into_inner(), &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match people::repository::get(id, &connection) {
        Ok(_) => people::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}*/
