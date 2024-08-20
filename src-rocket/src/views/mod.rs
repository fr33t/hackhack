#![allow(private_interfaces)]
pub mod auth;
pub mod target;

#[get("/")]
pub fn index() -> String {
    "Hello, world!".into()
}
