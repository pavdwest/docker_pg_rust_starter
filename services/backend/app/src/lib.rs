pub mod models;
pub mod schema;

use std::env;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use self::models::{NewPost, Post};

pub fn get_connection() -> PgConnection {
    dotenv().ok();
    let username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set!");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set!");
    let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set!");
    let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set!");
    let database_url = format!(
        "postgres://{username}:{password}@{host}/{name}"
    );
    println!("database_url: {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
