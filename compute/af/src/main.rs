extern crate uuid;
extern crate lettre;

use std::env;
use std::net::Ipv4Addr;
use warp::{Filter};

use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    
    let af = warp::post()
            .and(warp::path("api"))
            .and(warp::path("send-email"))
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .and_then(send_email);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(af).run((Ipv4Addr::LOCALHOST, port)).await
}

// SEND EMAIL SECTION
#[derive(Deserialize)]
pub struct EmailMessageModel {
    pub name: String,
    pub email: String,
    pub body: String
}

async fn send_email(email_message: EmailMessageModel) -> Result<impl warp::Reply, warp::Rejection> {
    let server = env::var("SERVER_SMTP").expect("SERVER_SMTP must be set");
    let username_smtp = env::var("USERNAME_SMTP").expect("USERNAME_SMTP must be set");
    let password_smtp = env::var("PASSWORD_SMTP").expect("PASSWORD_SMTP must be set");
    let to_email = env::var("TO_EMAIL").expect("TO_EMAIL must be set");

    let sender = SmtpTransport::starttls_relay(&server)
                    .expect("Failed to create transport")
                    .credentials(Credentials::new(username_smtp.to_string(), password_smtp.to_string()))
                    // Configure expected authentication mechanism
                    .authentication(vec![Mechanism::Plain])
                    // Connection pool settings
                    .pool_config(PoolConfig::new().max_size(20))
                    .build();

    let email = Message::builder()
                    .from(format!("NoBody <{}>", username_smtp).parse().unwrap())
                    .reply_to(format!("{} <{}>", email_message.name, email_message.email).parse().unwrap())
                    .to(format!("Web Form <{}>", to_email).parse().unwrap())
                    .subject("Interesado en colaborar contigo")
                    .body(String::from(email_message.body))
                    .unwrap();

    let result = sender.send(&email);
    
    if result.is_ok() {
        return Ok(warp::reply());
    }

    return Err(warp::reject());
}