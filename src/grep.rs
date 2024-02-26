use regex::Regex;
use std::error::Error;
use std::{
    fs,
    io::{self, BufRead, BufReader},
};

pub fn grep<S: AsRef<str>>(
    pattern: S,
    file: S,
    mut out: impl io::Write,
) -> Result<(), Box<dyn Error>> {
    let open_file = fs::File::open(file.as_ref())?;
    let reader = BufReader::new(open_file);

    let re = Regex::new(pattern.as_ref())?;

    for line_result in reader.lines() {
        let line = line_result?;

        if re.is_match(line.as_str()) {
            writeln!(&mut out, "{}", line)?;
        }
    }

    Ok(())
}

macro_rules! grep {
    ($pattern:expr, $file:expr, $out:expr) => {
        grep($pattern, $file, $out)
    };
    ($pattern:expr, $file:expr) => {
        grep::grep($pattern, $file, io::stdout())
    };
}
