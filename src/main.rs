mod file;
use file::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new();
    let file_data = file_handler.import("lorem_ipsum.txt").unwrap();
    let data = &file_data.inner;
    println!("{}", data);
    Ok(())    
}
