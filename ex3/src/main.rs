#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

#[post("/receive", data="<message>")]
fn receive (message: String) -> String {
    match message.trim() {
        "I eat yellow snow!" => format!("Don't do that!"),
        _ => format!("Response received: {}", message),
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![receive])
}