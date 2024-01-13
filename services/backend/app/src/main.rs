// https://www.workfall.com/learning/blog/how-to-create-a-rest-api-with-rust-rocket-framework-and-diesel-middleware-with-postgresql-database/
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![hello])
}
