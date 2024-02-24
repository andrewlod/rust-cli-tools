use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;
use std::error;

fn find_recursive(path: Arc<str>, file: Arc<str>) -> Result<(), Box<io::Error>> {
    let path_ref = path.as_ref();
    let file_ref = file.as_ref();

    let paths = fs::read_dir(path_ref)?;

    let mut handles = vec![];

    for path_result in paths {
        let obj = path_result?;

        let obj_name = match obj.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => return Err(io::Error::new(io::ErrorKind::Unsupported, "Something went wrong while converting object name to UTF-8 string!"))?
        };

        if obj_name == file_ref.to_string() {
            println!("{}/{}", path_ref, file_ref);
            return Ok(());
        }

        let obj_type = obj.file_type()?;

        if obj_type.is_dir() {
            let path_ref_clone = Arc::clone(&path);
            let file_ref_clone = Arc::clone(&file);

            let handle = thread::spawn(move || -> Result<(), Box<io::Error>> {
                let new_path = &format!("{}/{}", path_ref_clone, obj_name);
                let new_path_ptr: Arc<str> = Arc::from(new_path.as_str());

                match find_recursive(new_path_ptr, file_ref_clone) {
                    Ok(v) => return Ok(v),
                    Err(e) => return Err(e)
                }
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

pub fn find<S: AsRef<str>>(path: S, file: S) -> Result<(), Box<dyn error::Error>> {
    let path_ptr: Arc<str> = Arc::from(path.as_ref());
    let file_ptr: Arc<str> = Arc::from(file.as_ref());

    find_recursive(path_ptr, file_ptr)?;

    Ok(())
}