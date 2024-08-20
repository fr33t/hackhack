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
        score INTEGER NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )",
        [],
    )
    .expect("Error creating table [user]");

    // Table [target]
    // INSERT INTO "main"."target" ("name", "ip", "level", "user", "pwned", "user_flag", "flag", "type", "description", "status") VALUES ('first', '10.10.0.10', '0', '0', '0', 'flag{7f36aa86-9c13-d4fd-cef4-ad6e0b4c8b61}', 'flag{b5b852f4-e068-621c-8526-e799a65d2136}','linux','This is a every simple target.','activate');
    conn.execute(
        "CREATE TABLE IF NOT EXISTS target (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        ip TEXT NOT NULL UNIQUE,
        level INTEGER NOT NULL,
        user INTEGER NOT NULL,
        type TEXT NOT NULL,
        description TEXT,
        pwned INTEGER NOT NULL,
        user_flag TEXT NOT NULL,
        flag TEXT NOT NULL,
        status TEXT NOT NULL
    )",
        [],
    )
    .expect("Error creating table [target]");

    // Table [log]
    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY,
            email INTEGER NOT NULL,
            flag_type TEXT NOT NULL,
            target_name TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .expect("Error creating table [log]");
}
