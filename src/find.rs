use std::error;
use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn find_recursive<W: io::Write + Send + 'static>(
    path: Arc<str>,
    file: Arc<str>,
    out: Arc<Mutex<W>>,
) -> Result<(), Box<io::Error>> {
    let path_ref = path.as_ref();
    let file_ref = file.as_ref();

    let paths = fs::read_dir(path_ref)?;

    let mut handles = vec![];

    for path_result in paths {
        let obj = path_result?;

        let obj_name = match obj.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Something went wrong while converting object name to UTF-8 string!",
                ))?
            }
        };

        if obj_name == file_ref {
            let mut out_lock = match out.lock() {
                Ok(v) => v,
                Err(_e) => {
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        "Something went wrong while locking the output mutex!",
                    )))
                }
            };

            writeln!(&mut out_lock, "{}/{}", path_ref, file_ref)?;
            return Ok(());
        }

        let obj_type = obj.file_type()?;

        if obj_type.is_dir() {
            let path_ref_clone = Arc::clone(&path);
            let file_ref_clone = Arc::clone(&file);
            let out_clone = Arc::clone(&out);

            let handle = thread::spawn(move || -> Result<(), Box<io::Error>> {
                let new_path = &format!("{}/{}", path_ref_clone, obj_name);
                let new_path_ptr: Arc<str> = Arc::from(new_path.as_str());

                match find_recursive(new_path_ptr, file_ref_clone, out_clone) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
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

pub fn find<S: AsRef<str>, W: io::Write + Send + 'static>(
    path: S,
    file: S,
    out: Arc<Mutex<W>>,
) -> Result<(), Box<dyn error::Error>> {
    let path_ptr: Arc<str> = Arc::from(path.as_ref());
    let file_ptr: Arc<str> = Arc::from(file.as_ref());

    find_recursive(path_ptr, file_ptr, out)?;

    Ok(())
}

pub fn find_stdout<S: AsRef<str>>(path: S, file: S) -> Result<(), Box<dyn error::Error>> {
    let out = Arc::new(Mutex::new(io::stdout()));
    find(path, file, out)
}

macro_rules! find {
    ($path:expr, $file:expr, $out:expr) => {
        find($path, $file, $out)
    };
    ($path:expr, $file:expr) => {
        find::find_stdout($path, $file)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let output_ptr = Arc::new(Mutex::new(Vec::new()));
        find!(".", "test_file.txt", Arc::clone(&output_ptr)).expect("Command failed!");

        let output = match output_ptr.lock() {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        let output_value = String::from_utf8(output.to_vec()).expect("Failed to convert to UTF-8!");
        let expected_value = "./assets/test_file.txt\n";

        assert_eq!(output_value, expected_value);
    }
}
