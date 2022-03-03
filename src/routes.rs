use rocket::{response::{Redirect}, request::Form};
use rocket_contrib::templates::Template;
use std::io;

use crate::page::{Page, SaveForm};


#[get("/view/<title>")]
pub fn view(title: String)-> Result<Template, Redirect> {
    if let page = Page::load(title.clone()).unwrap() {
        let response = Template::render("index", page);
        Ok(response)
    } else {
        Err(Redirect::to(uri!(create_page: "test_01")))
    }

}

#[get("/create/<title>")]
pub fn create_page(title: String)-> Template {
    let page = Page::load(title.clone()).unwrap();
    let response = Template::render("blank", page);
    response
}

#[post("/save/<title>", data = "<form>")]
pub fn save_page(title: String, form: Form<SaveForm>)-> io::Result<Redirect> {
    let form = form.into_inner();
    let page = Page {
        title: title.clone(),
        body: form.body,
    };

    page.save();

    Ok(Redirect::to(uri!(view: title)))
} 