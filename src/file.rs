use std::fs::{File};
use std::io::Read;

pub struct FileHandler {
    pub inner: String,
}

impl FileHandler {
    pub fn new() -> FileHandler {
        FileHandler {
            inner:String::new()
        }
    }
}

impl FileHandler {
    pub fn import(&mut self, file_url: &str) -> Result<FileHandler, Box<dyn std::error::Error>> {
        let mut file = File::open(file_url)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(FileHandler {
            inner: contents
        })
    }
}