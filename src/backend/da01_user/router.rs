// use connection;
use crate::backend::handler_da01;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        // .manage(connection::init_pool())
        .mount(
            "/api/da01_user",
            routes![
                // handler::all,
                handler_da01::hello_world,
                /*
                people::handler::get,
                people::handler::post,
                people::handler::put,
                people::handler::delete
                */
            ],
        )
        .launch();
}
