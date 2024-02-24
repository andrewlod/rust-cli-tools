use std::fs;

pub fn cat(args: Vec<&String>) {
    for path in args {
        let result = fs::read_to_string(path);

        let file_content = match result {
            Ok(v) => v,
            Err(e) => panic!("Could not get file content of file {}: {}", path, e)
        };

        println!("{}", file_content);
    }
}