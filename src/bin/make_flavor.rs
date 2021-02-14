extern crate diesel;

use std::io::{stdin, Read};

fn main() {
    let connection = secret_sauce::establish_connection();

    println!("What is the name of your new flavor?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("\nOk! Let's write {} (Press {} when finished)\n", name, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = secret_sauce::create_flavor(&connection, name, &body);
    println!("\nSaved draft {:?} with id {:?}", name, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
