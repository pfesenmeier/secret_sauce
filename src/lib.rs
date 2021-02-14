use self::models::{NewPost, Post};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_flavor<'a>(
    conn: &PgConnection,
    name: &'a str,
    description: &'a str,
) -> models::Flavor {
    let new_flavor = models::NewFlavor { name, description };

    diesel::insert_into(schema::flavors::table)
        .values(&new_flavor)
        .get_result(conn)
        .expect("Error saving new flavor")
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}