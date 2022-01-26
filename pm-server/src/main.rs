#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::FromParam;
use rocket::response::{content, status};
use rocket::Request;

#[derive(Debug)]
enum Source {
    Github,
}

impl<'r> FromParam<'r> for Source {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match param {
            "github" => Ok(Source::Github),
            _ => Err("Did not match source"),
        }
    }
}

#[post("/sync/<source>")]
fn sync(source: Source) -> &'static str {
    println!("{source:?}");
    return "sync";
}

#[catch(404)]
fn general_not_found() -> content::Json<&'static str> {
    content::Json(
        r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#,
    )
}
#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[launch]
fn rocket() -> _ {
    // let db = Database::new();
    rocket::build()
        .mount("/", routes![sync])
        .register("/", catchers![general_not_found, default_catcher])
}
