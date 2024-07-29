extern crate uuid;
extern crate lettre;
extern crate native_tls;

use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

use lettre::{SendableEmail, EmailAddress, EmailTransport};
use lettre::smtp::{SmtpTransportBuilder, SUBMISSION_PORT};
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::ClientTlsParameters;

use native_tls::TlsConnector;

#[tokio::main]
async fn main() {
    sendEmail("Web Form", "j.louis1493@gmail.com"); 
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("send-email"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
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
struct CrashReport {
    to: Vec<EmailAddress>,
    from: EmailAddress,
    message_id: String,
    message: Vec<u8>,
}

// A simple constructor for our email.
impl CrashReport {
    pub fn new(from_address: EmailAddress,
        to_addresses: Vec<EmailAddress>,
        message_id: String,
        message: String) -> CrashReport {
            CrashReport { from: from_address,
            to: to_addresses,
            message_id: message_id,
            message: message.into_bytes()
            }
        }
}

impl<'a> SendableEmail<'a, &'a [u8]> for CrashReport {
    fn to(&self) -> Vec<EmailAddress> {
        self.to.clone()
    }

    fn from(&self) -> EmailAddress {
        self.from.clone()
    }

    fn message_id(&self) -> String {
        self.message_id.clone()
    }

    fn message(&'a self) -> Box<&[u8]> {
        Box::new(self.message.as_slice())
    }
}

fn sendEmail(emisor: String, email: String){
    let server = "smtp.ionos.mx";
    let connector = TlsConnector::builder().unwrap().build().unwrap();
    let mut transport = SmtpTransportBuilder::new((server, SUBMISSION_PORT), lettre::ClientSecurity::Opportunistic(<ClientTlsParameters>::new(server.to_string(), connector)))
                        .expect("Failed to create transport")
                        .credentials(Credentials::new(env::var("USERNAME").unwrap_or_else(|_| "contacto@luispsarmiento.com".to_string()), env::var("PASSWORD").unwrap_or_else(|_| "pass".to_string())))
                        .build();
    let report = CrashReport::new(EmailAddress::new(email.to_string()), vec![EmailAddress::new("contacto@luispsarmiento.com".to_string())], "foo".to_string(), "OOPS!".to_string());
    transport.send(&report).expect("Failed to send the report");
}