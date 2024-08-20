#[macro_use]
extern crate rocket;
use rocket_cors::AllowedOrigins;
use rocket_cors::CorsOptions;

// custom module
mod table;
mod verify;
mod views;

// import all views
use views::*;

#[launch]
fn rocket() -> _ {
    println!("Current: {:?}", std::env::current_dir().unwrap());

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount(
            "/auth",
            routes![auth::get_code, auth::check_login, auth::test_login],
        )
        .mount("/target", routes![target::all, target::log])
}
