extern crate uuid;
extern crate lettre;

use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::PoolConfig;

#[tokio::main]
async fn main() {
    
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("send-email"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            //sendEmail("Web Form".to_string(), "j.louis1493@gmail.com".to_string());
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

// SEND EMAIL SECTION
fn sendEmail(emisor: String, emisorEmail: String){
    let server = "smtp.ionos.mx";

    let sender = SmtpTransport::starttls_relay(server)
                    .expect("Failed to create transport")
                    .credentials(Credentials::new("contacto@luispsarmiento.com".to_string(), "pass".to_string()))
                    // Configure expected authentication mechanism
                    .authentication(vec![Mechanism::Plain])
                    // Connection pool settings
                    .pool_config(PoolConfig::new().max_size(20))
                    .build();

    let email = Message::builder()
                    .from("NoBody <contacto@luispsarmiento.com>".parse().unwrap())
                    .reply_to("Some <j.louis1493@gmail.com>".parse().unwrap())
                    .to("Yuin <sm@luispsarmiento.com>".parse().unwrap())
                    .subject("Happy new year")
                    .body(String::from("Be happy!"))
                    .unwrap();

    let result = sender.send(&email);
    result.expect("Failed to send the report");
}