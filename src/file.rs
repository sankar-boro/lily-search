use std::fs::{File};
use std::io::Read;

use crate::string::*;

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
    pub fn read_from(&mut self, file_url: &str) -> Result<FileHandler, Box<dyn std::error::Error>> {
        let mut file = File::open(file_url)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(FileHandler {
            inner: contents
        })
    }

    pub fn get_split_string(&mut self, file_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let data = self.read_from(file_url)?;
        let data = &data.inner;
        let split_data: Vec<String> = split_string::<Vec<String>>(data);
        Ok(split_data)
    }
}