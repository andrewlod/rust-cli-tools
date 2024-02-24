use regex::Regex;
use std::{fs, io::{BufRead, BufReader}};

pub fn grep<S: AsRef<str>>(pattern: S, file: S) {
    let open_file_result = fs::File::open(file.as_ref());
    let open_file = match open_file_result {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let regex_result = Regex::new(pattern.as_ref());
    let re = match regex_result {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let reader = BufReader::new(open_file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        if re.is_match(line.as_str()) {
            println!("{}", line);
        }
    }
}