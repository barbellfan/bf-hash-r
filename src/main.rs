use rocket::fairing::{ Fairing, Info, Kind };
use rocket::http::Header;
use rocket::response::Responder;
use rocket::{ Request, Response };

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Cors)
    .mount("/", routes!(brute_force_hashcode))
}

#[get("/BruteForceHashcode")]
fn brute_force_hashcode() -> BruteForceHashcodeResponse {
    let bf = BruteForceHashcodeResponse {
        result_found: Some(String::from("a response")),
    };

    bf
}

struct BruteForceHashcodeRequest {
    hashcode: String,
    max_len: u32,
    include_digits: bool,
    include_lower_case: bool,
    include_upper_case: bool,
    include_punctuation: bool,
    include_white_space: bool,
}

#[derive(Responder)]
struct BruteForceHashcodeResponse {
    result_found: Option<String>,
}

pub struct Cors;

/// All hail stack overflow for this solution, if it works:
/// 
/// https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
/// 
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET")); response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
