use rusqlite::Connection;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

//WARN: unwrap shouldnt be used in production ready code, this is just a prototype
// opens sqlite db and tries to create a table if one doesnt already exists
pub fn db_init() -> Connection {
    let conn = Connection::open("./conjured.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .unwrap();
    conn
}

//WARN: unwrap shouldnt be used in production ready code, this is just a prototype
pub fn add_user(name: String, email: String, conn: &Connection) {
    let me = User {
        id: 0,
        name: name.to_owned(),
        email: email.to_owned(),
    };
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        (&me.name, &me.email),
    )
    .unwrap();
}

//WARN: unwrap shouldnt be used in production ready code, this is just a prototype
pub fn list_users(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name, email FROM users").unwrap();
    let user_iter = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            })
        })
        .unwrap();
    // I think it's kinda neat to see them without 'cating' the db file all the time
    for user in user_iter {
        println!("Found user: {:?}", user.unwrap());
    }
}
