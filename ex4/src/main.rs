#[macro_use]

extern crate rocket;
use rocket::form::{Form, FromForm};
use rocket::response::content::RawHtml;
#[derive(FromForm)]

struct UserInput {
    good: Option<String>,
    bad: Option<String>,
}

#[get("/")]

fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <div>
    <h1>How are you?</h1>
        <form action="/answer" method="post">
            <label for="good">Good</label>
            <input type="checkbox" id="good" name="good" value="good">
            <label for="bad">Bad</label>
            <input type="checkbox" id="bad" name="bad" value="bad">
            <input type="submit" value="Submit answer">
        </form>
    </div>
    "#)
}

#[post("/answer", data = "<answer>")]
fn answer(answer:Form<UserInput>) -> RawHtml<String> {
    let a = answer.into_inner();

    let message = match (a.good.is_some(), a.bad.is_some()) {
    (true, true) => "Can you really be having both a good and a bad day at the same time?",
    (true, false) => "Hey, I am glad to hear that. Keep on rockin'! :)",
    (false, true) => "I'm sorry to hear that. I hope things get better for you. :(",
    (false, false) => "You did not share your feelings. :(",
    };

    RawHtml(format!("<div id=\"response\"><h2>{}</h2></div>", message))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, answer])
}