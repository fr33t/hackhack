use crate::table::DB;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Target {
    name: String,
    level: i32,
    user: i32,
    pwned: i32,
    user_flag: String,
    flag: String,
}

#[get("/all")]
pub fn all() -> Json<Vec<Target>> {
    let db = DB.lock().expect("Cannot use database");
    let mut query_target_stmt = db
        .prepare("SELECT name, level, user, pwned FROM target;")
        .expect("SQL error");
    let targets = query_target_stmt
        .query_map([], |row| {
            Ok(Target {
                name: row.get(0).unwrap(),
                level: row.get(1).unwrap(),
                user: row.get(2).unwrap(),
                pwned: row.get(3).unwrap(),
                user_flag: String::new(),
                flag: String::new(),
            })
        })
        .expect("Extraction error");
    let mut res = Vec::new();
    for t in targets {
        res.push(t.unwrap());
    }
    Json(res)
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
    email: String,
    flag_type: String,
    target_name: String,
    created_at: String,
}
#[get("/log")]
pub fn log() -> Json<Vec<Log>> {
    let db = DB.lock().expect("Cannot use database");
    let mut query_target_stmt = db
        .prepare("SELECT email,flag_type,target_name,created_at FROM log;")
        .expect("SQL error");
    let targets = query_target_stmt
        .query_map([], |row| {
            Ok(Log {
                email: row.get(0).unwrap(),
                flag_type: row.get(1).unwrap(),
                target_name: row.get(2).unwrap(),
                created_at: row.get(3).unwrap(),
            })
        })
        .expect("Extraction error");
    let mut res = Vec::new();
    for t in targets {
        res.push(t.unwrap());
    }
    Json(res)
}
