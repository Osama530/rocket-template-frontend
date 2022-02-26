use rocket::response::{content::Html, self, Redirect};
use rocket_contrib::templates::Template;
use std::io;
use super::Page;

#[get("/view/<title>")]
pub fn view(title: String)-> Result<Template, Redirect> {
    if let Ok(page) = Page::load(title.clone()) {
        let res = Template::render("view", page);
        Ok(res)
    } else {
        Err(Redirect::to(uri!(edit: title)))
    }

}

#[get("/edit/<title>")]
pub fn edit(title: String)-> Html<String> {
    let page = Page::load(title.clone()).unwrap_or(Page::blank(title));

    let response = format!("
    <h1>Editing<h1>
    <form action=\"/save/{title}\" method=\"POST\">
            <textarea name=\"body\">{body}</textarea><br>
            <input type=\"submit\" value=\"Save\">
    </form>", title = page.title, body = page.body );
    Html(response)
}

#[get("/edit_tera/<title>")]
pub fn edit_tera(title: String)-> Template {
    let page = Page::load(title.clone()).unwrap_or(Page::blank(title));

    Template::render("edit", page)

}