#[macro_use] extern crate rocket;

#[get("/BruteForceHashcode")]
fn brute_force_hashcode() -> &'static str {
    "got it!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes!(brute_force_hashcode))
}
