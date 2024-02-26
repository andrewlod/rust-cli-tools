use std::error::Error;
use std::{fs, io};

pub fn cat(args: Vec<&String>, mut out: impl io::Write) -> Result<(), Box<dyn Error>> {
    for path in args {
        let file_content = fs::read_to_string(path)?;

        writeln!(&mut out, "{}", file_content)?;
    }

    Ok(())
}

macro_rules! cat {
    ($args:expr, $out:expr) => {
        cat($args, $out)
    };
    ($args:expr) => {
        cat::cat($args, io::stdout())
    };
}
