use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use rand::Rng;
pub fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(0..100000);

    format!("{:05}", number)
}

pub fn send_email(target_email: &str, verify_code: &str) {
    let mine_email = "freetbash@163.com";
    let passwd = "SFSCXPSEMGRUJRIP";
    let smtp_server = "smtp.163.com";

    let email = Email::builder()
        .to(target_email)
        .from(mine_email)
        .subject("hackhack")
        .html(format!("<h1>{}</h1>", verify_code))
        .build()
        .unwrap();

    let creds = Credentials::new(mine_email.to_string(), passwd.to_string());
    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
    print!("{:?}", result);
    mailer.close();
}
