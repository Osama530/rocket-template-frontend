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
    let page_01 = Page {
        title: "test_01".to_string(),
        body: "here is the body for test page 01".to_string()
    };

    page_01.save();
    
    let page_02 = Page {
        title: "test_02".to_string(),
        body: "here is the body for test page 02".to_string()
    };

    page_02.save();

    rocket::ignite()
        .mount("/", routes![view, save_page])
        .attach(Template::fairing())
        .launch();
    

}
