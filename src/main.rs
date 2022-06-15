mod file;
use file::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_handler = FileHandler::new();
    let data = file_handler.get_split_string("lorem_ipsum.txt").unwrap();
    println!("{:?}", data);
    Ok(())    
}
