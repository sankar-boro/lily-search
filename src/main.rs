mod file;
mod string;
mod db;

use file::*;
use db::*;
use string::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new();
    let data: FileHandler = file_handler.read_from("lorem_ipsum.txt").unwrap();
    let string_small = &data.inner.to_lowercase();
    let clean_data = clean_string(&string_small);
    // let db = ignore_strings(&data.inner);
    println!("{:?}", clean_data);
    Ok(())    
}
