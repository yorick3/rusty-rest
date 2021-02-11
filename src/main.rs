#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> &'static str {
    "RustyRest is running!"
}

#[get("/makestring/<input>")]
fn makestring(input: String) -> String {
    format!("You entered: '{}'", input)
}

#[get("/image")]
fn image() -> Option<NamedFile> {
    NamedFile::open("rusty.jpg").ok()
}

#[get("/video")]
fn video() -> Option<NamedFile> {
    NamedFile::open("video.mp4").ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, makestring, image, video]).launch();
}