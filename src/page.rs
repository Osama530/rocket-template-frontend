use std::fs::File;
use std::io::{self, Read, Write};
use std::file;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub body: String,
}

impl Page {
    pub fn load(title: String)-> io::Result<Page> {
        let file_name = format!("{}.txt",title);
        let mut file = File::open(file_name)?;

        let mut body = String::new();
        file.read_to_string(&mut body)?;

        Ok(Page {
            title,
            body,
        })
    }

    pub fn save(&self)-> io::Result<()> {
        let file_name = format!("{}.txt", self.title);
        let mut file = File::create(file_name)?;
        write!(file, "{}", self.body)
    }
    
    pub fn create(title: String)-> Page {
        Page{
            title,
            body: "please create a page first".to_string()
        }
    }
}

//url-encoded form
#[derive(Debug, FromForm)]
pub struct SaveForm {
    pub body: String
}


