use std::thread;

use crate::{table::DB, verify};
use rocket::serde::json::{serde_json::json, Value};
use rocket::{form::Form, FromForm};
use rusqlite::params;
#[derive(FromForm)]
struct UserEmail<'a> {
    email: &'a str,
}

#[post("/get_code", data = "<user_email>")]
pub fn get_code(user_email: Form<UserEmail>) -> Value {
    let db = DB.lock().expect("Cannot use database");

    // judge the if haven sent the code
    let mut sent_exist_stmt = db
        .prepare("SELECT EXISTS(SELECT 1 FROM verify WHERE email = ?1);")
        .expect("SQL Error");
    let sent_exist: bool = sent_exist_stmt
        .query_row(params![user_email.email], |row| row.get(0))
        .expect("Query Error");

    if sent_exist {
        return json!({"status":200,"message":"You've sent the code, please wait a minute."});
    }

    // check user exists
    let mut user_exist_stmt = db
        .prepare("SELECT EXISTS(SELECT 1 FROM user WHERE email = ?1);")
        .expect("SQL Error");
    let user_exists: bool = user_exist_stmt
        .query_row(params![user_email.email], |row| row.get(0))
        .expect("Query Error");
    if !user_exists {
        db.prepare("INSERT INTO user (email) VALUES (?1);")
            .expect("SQL Error")
            .execute(params![user_email.email])
            .expect("Insert Error");
    }

    // generate and send verify code to user email and check db
    let verify_code = verify::generate_code();
    db.prepare("INSERT INTO verify (email, code) VALUES (?1, ?2);")
        .expect("SQL Error")
        .execute(params![user_email.email, &verify_code])
        .expect("Insert Error");

    verify::send_email(user_email.email, &verify_code);

    // auto delete task
    let delete_email = user_email.email.to_string();
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(60)); // 1 minutes
        let db = DB.lock().expect("Cannot use database");
        db.prepare("DELETE FROM verify WHERE email =?;")
            .expect("SQL Error")
            .execute(params![delete_email])
            .expect("Delete Error");
    });

    return json!({"status":201, "message":"The verifyCode has been sent to your email, pls check it!"});
}

#[derive(FromForm)]
struct UserVerify<'a> {
    email: &'a str,
    code: &'a str,
}
#[post("/check_login", data = "<user_verify>")]
pub fn check_login(user_verify: Form<UserVerify>) -> Value {
    let db = DB.lock().expect("Cannot use database");

    // check auth
    let mut code_exist_stmt = db
        .prepare("SELECT EXISTS(SELECT 1 FROM verify WHERE email = ?1 AND code = ?2);")
        .expect("SQL Error");
    let code_exist: bool = code_exist_stmt
        .query_row(params![user_verify.email, user_verify.code], |row| {
            row.get(0)
        })
        .expect("Query Error");

    // login and delete
    if code_exist {
        db.prepare("DELETE FROM verify WHERE email =?;")
            .expect("SQL Error")
            .execute(params![user_verify.email])
            .expect("Delete Error");

        return json!({"status":200, "message":"Login Success!"});
    } else {
        return json!({"status":401, "message":"Login Failure!"});
    }
}
