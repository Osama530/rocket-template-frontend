use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write, self};

use serde::Serialize;

#[derive(Serialize,Debug, PartialEq, Eq)]
pub struct Page {
    pub title : String,
    pub body : String,
}

impl Page {
    pub fn load(title: String)-> io::Result<Page> {
        let file_name = format!("{}.txt",title);
        let mut file = File::open(file_name)?;
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        Ok( Page {  title, body })
    }

    pub fn save(&self)-> io::Result<()> {
        let file_name = format!("{}.txt", self.title);
        let mut file = File::create(file_name)?;
        write!(file, "{}", self.body)
    }

    pub fn blank(title: String)-> Page {
        Page {
            title,
            body : String::new()
        }
    }
}