#![allow(private_interfaces)]
pub mod auth;

#[get("/")]
pub fn index() -> String {
    "Hello, world!".into()
}
