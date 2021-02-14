extern crate diesel;
extern crate secret_sauce;

use self::diesel::prelude::*;
use self::models::*;
use self::secret_sauce::*;

fn main() {
    use secret_sauce::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
