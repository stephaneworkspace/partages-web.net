use crate::backend::handler_da01;
use crate::backend::handler_db01;
use crate::backend::handler_html;
use crate::connection;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/api/da01_user",
            routes![handler_da01::all, handler_da01::hello_world,],
        )
        .mount("/api/db01_quote", routes![handler_db01::all])
        .mount("/", routes![handler_html::index, handler_html::all])
        .launch();
}
