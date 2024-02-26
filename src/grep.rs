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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep() {
        let mut output = Vec::new();
        grep!("and", "assets/test_file.txt", &mut output).expect("Command failed!");

        let output = String::from_utf8(output).expect("Failed to convert to UTF-8!");
        let expected_value = "The stained glass window had slivers of yellow, orange, and red.
My mom made a milkshake with frozen bananas and chocolate sauce.
It is illegal to buy and sell tigers and other big cats in the United States.
";

        assert_eq!(output, expected_value);
    }
}
