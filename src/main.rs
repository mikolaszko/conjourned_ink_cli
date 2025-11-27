use rusqlite::Result;
use std::env;

use crate::user_commands::{add_user, db_init, list_users};

pub mod user_commands;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let email = &args[2];

    let connection = db_init();
    // I am taking shortcuts here
    add_user(name.to_owned(), email.to_owned(), &connection);
    list_users(&connection);

    Ok(())
}
