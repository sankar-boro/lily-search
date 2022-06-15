mod file;
mod string;
mod db;

use file::*;
use db::*;
use string::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new();
    let data: Vec<String> = file_handler.get_split_string("lorem_ipsum.txt").unwrap();
    let db = ignore_strings(&data);
    println!("{:?}", db);
    Ok(())    
}
