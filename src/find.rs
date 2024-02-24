use std::fs;
use std::thread;
use std::sync::Arc;

fn find_recursive(path: Arc<str>, file: Arc<str>) {
    let path_ref = path.as_ref();
    let file_ref = file.as_ref();

    let read_dir_result = fs::read_dir(path_ref);

    let paths = match read_dir_result {
        Ok(v) => v,
        Err(e) => panic!("Could not read directory. Reason: {}", e) 
    };

    let mut handles = vec![];

    for path_result in paths {
        let obj = match path_result {
            Ok(v) => v,
            Err(e) => panic!("Something went wrong while reading a directory: {}", e)
        };

        let obj_name = match obj.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => panic!("Something went wrong while converting directory name into a string!")
        };

        if obj_name == file_ref.to_string() {
            println!("{}/{}", path_ref, file_ref);
            return;
        }

        let obj_type = match obj.file_type() {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        if obj_type.is_dir() {
            let path_ref_clone = Arc::clone(&path);
            let file_ref_clone = Arc::clone(&file);

            let handle = thread::spawn(move || {
                let new_path = &format!("{}/{}", path_ref_clone, obj_name);
                let new_path_ptr: Arc<str> = Arc::from(new_path.as_str());
                find(new_path_ptr, file_ref_clone);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

}

pub fn find<S: AsRef<str>>(path: S, file: S) {
    let path_ptr: Arc<str> = Arc::from(path.as_ref());
    let file_ptr: Arc<str> = Arc::from(file.as_ref());

    find_recursive(path_ptr, file_ptr);
}