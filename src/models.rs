use super::schema::flavors;
use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable)]
pub struct Flavor {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "flavors"]
pub struct NewFlavor<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
