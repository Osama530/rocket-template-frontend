// https://fediverse.blog/~/3542/following-the-go-tutorial-on-making-a-wiki-in-rust-using-rocket/
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod page;
mod routes;

use page::Page;
use rocket_contrib::templates::Template;
use routes::*;
fn main() {
    let page =  Page {
        title: String::from("TEST"),
        body: String::from("This is the page"),
    };

    page.save();

    rocket::ignite()
        .mount("/", routes![view, edit, edit_tera])
        .attach(Template::fairing())
        .launch();

}
