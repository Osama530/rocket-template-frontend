// https://fediverse.blog/~/3542/following-the-go-tutorial-on-making-a-wiki-in-rust-using-rocket/
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod page;
mod routes;

use page::Page;
use routes::*;
use rocket::fairing;
use rocket_contrib::templates::Template;

fn main() {

    rocket::ignite()
        .mount("/", routes![index, save, success])
        .attach(Template::fairing())
        .launch();
    

}
