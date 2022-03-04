use rocket::{request::Form, response::Redirect};
use rocket_contrib::templates::Template;

use crate::page::{Page};

#[get("/index")]
pub fn index()->Template {
    let context = Page::blank();
    Template::render("index", context)
}

#[post("/save", data = "<form>")]
pub fn save(form: Form<Page>)-> Redirect{
    let form_data = form.into_inner();
    
    let page = Page::create(form_data.title, form_data.body);
    Page::save(&page);

    Redirect::to(uri!(success))
}

#[get("/success")]
pub fn success()-> Template {
    let content = Page::blank();
    Template::render("success", content)
}
