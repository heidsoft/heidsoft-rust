#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// https://rocket.rs/v0.5-rc/guide/getting-started/
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}