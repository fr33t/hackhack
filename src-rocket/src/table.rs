use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub static DB: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| Arc::new(Mutex::new(init_db())));

fn init_db() -> Connection {
    let conn = Connection::open("hackhack.db").expect("Failed to open database!");
    // init table
    init_table(&conn);
    conn
}

fn init_table(conn: &Connection) {
    // Table [verify]
    conn.execute(
        "CREATE TABLE IF NOT EXISTS verify (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            code TEXT NOT NULL
        )",
        [],
    )
    .expect("Error creating table [verify]");

    // Table [user]
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY,
        email TEXT NOT NULL UNIQUE,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )",
        [],
    )
    .expect("Error creating table [user]");
}
