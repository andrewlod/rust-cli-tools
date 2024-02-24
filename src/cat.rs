use std::fs;
use std::error::Error;

pub fn cat(args: Vec<&String>) -> Result<(), Box<dyn Error>> {
    for path in args {
        let file_content = fs::read_to_string(path)?;

        println!("{}", file_content);
    }

    Ok(())
}