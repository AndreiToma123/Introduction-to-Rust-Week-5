#[macro_use] extern crate rocket;

#[get("/hello/<parameter>")]
fn hello (parameter: String) -> String {
    format!("Hello, {}!", parameter)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![hello])
}