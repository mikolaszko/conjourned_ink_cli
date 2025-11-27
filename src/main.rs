use rusqlite::{Connection, Result};
use std::{env, path::Path};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let email = &args[2];
    let conn = Connection::open("./my.db")?;
    if !Path::exists(Path::new("./my.db")) {
        conn.execute(
            "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL
        )",
            (), // empty list of parameters.
        )?;
    }

    let me = User {
        id: 0,
        name: name.to_owned(),
        email: email.to_owned(),
    };
    conn.execute(
        "INSERT INTO person (name, email) VALUES (?1, ?2)",
        (&me.name, &me.email),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, email FROM person")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    // I think it's kinda neat to see them without 'cating' the db all the time
    for user in user_iter {
        println!("Found user {:?}", user.unwrap());
    }
    Ok(())
}
