use std::fs;
use std::error::Error;

pub fn ls<S: AsRef<str>>(path: S) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(path.as_ref())?;

    for path_result in paths {
        let dir = path_result?;

        let dir_name = match dir.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => return Err("Something went wrong while converting object name to UTF-8 string!")?
        };

        println!("{}", dir_name);
    }

    Ok(())
}