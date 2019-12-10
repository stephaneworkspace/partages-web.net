use backend::da01_user;
use connection;
use rocket;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/api/da01_user",
            routes![
                da01_user::handler::all,
                da01_user::handler::hello_world,
                /*
                da01_user::handler::get,
                da01_user::handler::post,
                da01_user::handler::put,
                da01_user::handler::delete
                */
            ],
        )
        .launch();
}
