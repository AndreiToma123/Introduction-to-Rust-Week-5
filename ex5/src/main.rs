#[macro_use]
extern crate rocket;
use rocket::form::{Form, FromForm};
use rocket::response::content::RawHtml;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(FromForm)]

struct Post {
    content: String,
}

#[post("/message", data = "<post>")]
fn post_message(post:Form<Post>) -> &'static str {
    let file_path = "data.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Could not open file");

    writeln!(file, "{}", post.content).expect("Could not write to file.");

    "Message received."
}

#[get("/message")]
fn get_message() -> RawHtml<String> {
    let file_path = "data.txt";

    if !Path::new(file_path).exists() {
        File::create(file_path).expect("Could not create file");
    }

    let mut file_content = String::new();
    let mut file = File::open(file_path).expect("Could not read file");
    file.read_to_string(&mut file_content).expect("Could not read file");

    RawHtml(format!("<p>{}</p>", file_content))
}

#[get("/")]
fn form_page() -> RawHtml<&'static str> {
    let render_page: &str = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Ch5 Pt5</title>
    </head>
    <body>
        <form action="/message" method="post">
            <label for="content">Post something</label>
            <input type="text" id="content" name="content">
            <input type="submit" value="Submit answer">
        </form>
    </body>
    </html>
    "#;

    RawHtml(render_page)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![form_page, get_message, post_message])
}