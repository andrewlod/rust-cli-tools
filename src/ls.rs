use std::error::Error;
use std::{fs, io};

pub fn ls<S: AsRef<str>>(path: S, mut out: impl io::Write) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(path.as_ref())?;

    for path_result in paths {
        let dir = path_result?;

        let dir_name = match dir.file_name().into_string() {
            Ok(v) => v,
            Err(_e) => {
                return Err("Something went wrong while converting object name to UTF-8 string!")?
            }
        };

        writeln!(&mut out, "{}", dir_name)?;
    }

    Ok(())
}

macro_rules! ls {
    ($path:expr, $out:expr) => {
        ls($path, $out)
    };
    ($path:expr) => {
        ls::ls($path, io::stdout())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        let mut output = Vec::new();
        ls!("assets", &mut output).expect("Command failed!");

        let output = String::from_utf8(output).expect("Failed to convert to UTF-8!");
        let expected_value = "test_file.txt\n";

        assert_eq!(output, expected_value);
    }
}
