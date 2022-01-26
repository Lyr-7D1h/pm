use diesel::{Connection, Queryable, SqliteConnection};

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub git: String,
    pub published: bool,
}

pub struct Database {}

impl Database {
    pub fn new() -> Database {
        SqliteConnection::establish("./pm.db")
            .unwrap_or_else(|_| panic!("Error connecting to {}", "./pm.db"));

        Database {}
    }
}
