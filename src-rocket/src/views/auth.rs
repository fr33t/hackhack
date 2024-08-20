use crate::{table::DB, verify};
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};
use rocket_jwt::jwt;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::thread;

#[jwt("freet", exp = 2592000)] // a month time
pub struct Token {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct UserEmail<'a> {
    email: &'a str,
}

#[post("/get_code", data = "<user_email>")]
pub fn get_code(user_email: Json<UserEmail>) -> Value {
    let db = DB.get().expect("Cannot use database");

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
        db.prepare("INSERT INTO user (email, score) VALUES (?1, 0);")
            .expect("SQL Error")
            .execute(params![user_email.email])
            .expect("Insert Error");
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            // new openvpn profile
            // std::env::current_dir().unwrap()
            let mut ovpn = std::env::current_dir().unwrap();
            ovpn.push("openvpn-install");
            ovpn.push("openvpn-install.sh");
            let cmd = ovpn.to_str().unwrap();
            // add x
            Command::new("chmod").arg("+x").arg(cmd).output().unwrap();
            let child = Command::new(cmd)
                .arg("--addclient")
                .arg(user_email.email)
                .output()
                .unwrap();
            let ls_list = String::from_utf8(child.stdout).unwrap();
            println!("{}", ls_list);
        }
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
        let db = DB.get().expect("Cannot use database");
        db.prepare("DELETE FROM verify WHERE email =?;")
            .expect("SQL Error")
            .execute(params![delete_email])
            .expect("Delete Error");
    });

    return json!({"status":201, "message":"The verifyCode has been sent to your email, pls check it!"});
}

#[derive(Serialize, Deserialize)]
struct UserVerify<'a> {
    email: &'a str,
    code: &'a str,
}
#[post("/check_login", data = "<user_verify>")]
pub fn check_login(user_verify: Json<UserVerify>) -> Value {
    let db = DB.get().expect("Cannot use database");

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
        let token = Token {
            email: user_verify.email.to_string(),
        };

        let token = Token::sign(token);
        return json!({"status":200, "message":"Login Success!", "token": token});
    } else {
        return json!({"status":401, "message":"Login Failure!"});
    }
}

#[post("/test_login")]
pub fn test_login(token: Token) -> Value {
    // query score from user
    let db = DB.get().expect("Cannot use database");
    let mut user_score_stmt = db
        .prepare("SELECT score FROM user WHERE email =?;")
        .expect("SQL Error");
    let user_score: i32 = user_score_stmt
        .query_row(params![token.email], |row| row.get(0))
        .expect("Query Error");
    json!({"email":token.email,"score":user_score})
}
use rocket::fs::NamedFile;
use std::path::Path;
#[get("/vpn")]
pub async fn vpn(token: Token) -> Option<NamedFile> {
    let m = token.email.replace(".", "_").replace("@", "_") + ".ovpn";
    let file_path = Path::new("/opt/openvpn").join(m);
    NamedFile::open(file_path).await.ok()
}
