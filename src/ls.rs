use std::fs;

pub fn ls<S: AsRef<str>>(path: S) {
    let read_dir_result = fs::read_dir(path.as_ref());
    
    let paths = match read_dir_result {
        Ok(value) => value,
        Err(e) => panic!("Could not read directory. Reason: {}", e) 
    };

    for path_result in paths {
        let dir = match path_result {
            Ok(v) => v,
            Err(e) => panic!("Something went wrong while reading a directory: {}", e)
        };

        let dir_name = match dir.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => panic!("Something went wrong while converting directory name into a string!")
        };

        println!("{}", dir_name);
    }
}