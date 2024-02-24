use regex::Regex;
use std::{fs, io::{BufRead, BufReader}};
use std::error::Error;

pub fn grep<S: AsRef<str>>(pattern: S, file: S) -> Result<(), Box<dyn Error>> {
    let open_file = fs::File::open(file.as_ref())?;
    let reader = BufReader::new(open_file);

    let re = Regex::new(pattern.as_ref())?;

    for line_result in reader.lines() {
        let line = line_result?;

        if re.is_match(line.as_str()) {
            println!("{}", line);
        }
    }

    Ok(())
}