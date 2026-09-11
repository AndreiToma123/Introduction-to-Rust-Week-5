#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;

#[get("/<page_name>")]
fn render_page (page_name: String) -> RawHtml<&'static str> {
    let content = match page_name.as_str() {
        "bob" => "<div><h1>Bob the Builder</h1><p>Can we fix it? Yes we can!</p></div>",
        "quote" => "<div><h1>Quote</h1><p>All your base are belong to us!</p></div>",
        _ => "<p>Sorry, the page is not available.</p>",
    };
    RawHtml(content)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/page",routes![render_page])
}