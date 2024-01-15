// https://www.workfall.com/learning/blog/how-to-create-a-rest-api-with-rust-rocket-framework-and-diesel-middleware-with-postgresql-database/
use app::get_connection;
use diesel::PgConnection;
use app::models::*;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/posts/seed")]
fn posts_seed() -> &'static str {
    let conn = &mut get_connection();

    for i in 0..5 {
        let t = &String::from(format!("title {}", i));
        let b = &String::from(format!("body {}", i));
        create_post(conn, t, b);
    }

    "OK"
}

pub fn create_post(conn: &mut PgConnection, _title: &str, _body: &str) -> Post {
    use app::schema::posts::dsl::*;
    use diesel::prelude::*;

    println!("Creating post...");

    let new_post = NewPost { title: _title, body: _body };

    diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

#[get("/posts")]
fn posts() -> &'static str {
    use app::schema::posts::dsl::*;
    use diesel::prelude::*;

    // https://diesel.rs/guides/getting-started
    let conn = &mut get_connection();
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("-----------\n");
        println!("{}", post.title);
        println!("{}", post.body);
        println!("-----------\n");
    }

    "Done"
}

#[launch]
fn rocket() -> _ {
    // Launch rocket
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![hello])
    .mount("/", routes![posts])
    .mount("/", routes![posts_seed])
}
