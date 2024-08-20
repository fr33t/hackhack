use crate::auth::Token;
use crate::table::DB;
use rocket::serde::json::Json;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Target {
    name: String,
    level: i32,
    user: i32,
    pwned: i32,
    typ: String,
    description: String,
    ip: String,
    user_flag: String,
    flag: String,
}

#[get("/all")]
pub fn all() -> Json<Vec<Target>> {
    let db = DB.lock().expect("Cannot use database");
    let mut query_target_stmt = db
        .prepare("SELECT name, level, user, pwned, type FROM target WHERE status='activate';")
        .expect("SQL error");
    let targets = query_target_stmt
        .query_map([], |row| {
            Ok(Target {
                name: row.get(0).unwrap(),
                level: row.get(1).unwrap(),
                user: row.get(2).unwrap(),
                pwned: row.get(3).unwrap(),
                typ: row.get(4).unwrap(),
                ip: String::new(),
                description: String::new(),
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

#[get("/get/<target_name>")]
pub fn get_target(target_name: &str) -> Json<Target> {
    let db = DB.lock().expect("Cannot use database");
    let mut query_target_stmt = db
        .prepare(
            "SELECT name, level, user, pwned, type, description, ip FROM target WHERE name=?1;",
        )
        .expect("SQL error");
    let target = query_target_stmt
        .query_row([target_name], |row| {
            Ok(Target {
                name: row.get(0).unwrap(),
                level: row.get(1).unwrap(),
                user: row.get(2).unwrap(),
                pwned: row.get(3).unwrap(),
                typ: row.get(4).unwrap(),
                description: row.get(5).unwrap(),
                ip: row.get(6).unwrap(),
                user_flag: String::new(),
                flag: String::new(),
            })
        })
        .expect("Extraction error");
    Json(target)
}

#[derive(Serialize, Deserialize, Debug)]
struct FlagLog {
    flag: String,
    flag_type: String,
    target_name: String,
}
#[post("/submit", data = "<flag_log>")]
pub fn submit_flag(token: Token, flag_log: Json<FlagLog>) -> String {
    // token.email, flag, flag_type, target_name
    let db = DB.lock().expect("Cannot use the database");
    // check the log if it exists
    let mut log_query_stmt = db
        .prepare(
            "SELECT EXISTS(SELECT 1 FROM log WHERE target_name=?1 AND flag_type=?2 AND email=?3);",
        )
        .expect("SQL error");
    if log_query_stmt
        .query_row(
            [
                flag_log.target_name.clone(),
                flag_log.flag_type.clone(),
                token.email.clone(),
            ],
            |row| row.get(0),
        )
        .expect("Query Error")
    {
        // ? has already been submitted
        return "You have already pwned this target!".to_string();
    }
    let mut score = 0;
    if flag_log.flag_type == "user" {
        let mut user_flag_query_stmt = db
            .prepare("SELECT EXISTS(SELECT 1 FROM target WHERE name=?1 AND user_flag=?2);")
            .expect("SQL error");
        if user_flag_query_stmt
            .query_row(
                [flag_log.target_name.clone(), flag_log.flag.clone()],
                |row| row.get(0),
            )
            .expect("Query Error")
        {
            score = 1;
        }
    } else {
        let mut root_flag_query_stmt = db
            .prepare("SELECT EXISTS(SELECT 1 FROM target WHERE name=?1 AND flag=?2);")
            .expect("SQL error");
        if root_flag_query_stmt
            .query_row(
                [flag_log.target_name.clone(), flag_log.flag.clone()],
                |row| row.get(0),
            )
            .expect("Query Error")
        {
            score = 2
        }
    }
    if score != 0 {
        let mut insert_log_stmt = db
            .prepare("INSERT INTO log (email, flag_type, target_name) VALUES (?1,?2,?3);")
            .expect("SQL error");
        insert_log_stmt
            .execute([
                token.email.clone(),
                flag_log.flag_type.clone(),
                flag_log.target_name.clone(),
            ])
            .expect("Insert Error");

        // update email score
        let mut update_score_stmt = db
            .prepare("UPDATE user SET score=score+?1 WHERE email=?2;")
            .expect("SQL error");
        update_score_stmt
            .execute(params![score, token.email])
            .expect("Update Error");
        if score == 2 {
            // update target pwned
            let mut update_pwned_stmt = db
                .prepare("UPDATE target SET pwned=pwned+1 WHERE name=?1;")
                .expect("SQL error");
            update_pwned_stmt
                .execute(params![flag_log.target_name])
                .expect("Update Error");
        }

        return "Flag was submitted successfully!".to_string();
    } else {
        return "Error flag was submitted!".to_string();
    }
}
