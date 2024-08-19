#[macro_use]
extern crate rocket;

// custom module
mod table;
mod verify;
mod views;

// import all views
use views::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/auth", routes![auth::get_code, auth::check_login])
}
