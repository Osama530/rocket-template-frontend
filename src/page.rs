use std::fs::File;
use std::io::{self, Write};

use rocket::response::Redirect;
use serde::Serialize;

// #[derive(Debug, FromForm)]
// pub struct SaveForm {
//     pub title: String,
//     pub body: String,
// }

#[derive(Debug, Serialize, FromForm)]
pub struct  Page {
    pub title: String,
    pub body: String,
}

impl Page {

    pub fn blank()-> Page {
        Page {
            title: String::new(),
            body: String::new()
        }
    }

    pub fn create(title: String, body: String)-> Page {
        Page {
            title,
            body,
        }
    }

    pub fn save(&self)-> io::Result<()> {
        let file_name = format!("{}.txt",self.title);
        let mut file = File::create(file_name)?;
        write!(file,"{}",self.body)        
    }
}
