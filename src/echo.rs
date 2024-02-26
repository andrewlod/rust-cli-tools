use std::{error::Error, io};

pub fn echo<S: AsRef<str>>(args: Vec<S>, mut out: impl io::Write) -> Result<(), Box<dyn Error>> {
    let args_size = args.len();

    if args_size < 1 {
        Err("Echo: at least 1 argument must be provided!")?;
    }

    for (i, arg) in args.iter().enumerate() {
        let arg_ref = arg.as_ref();
        if i < args_size - 1 {
            write!(&mut out, "{} ", arg_ref)?;
            continue;
        }

        write!(&mut out, "{}", arg_ref)?;
    }

    Ok(())
}

macro_rules! echo {
    ($args:expr, $out:expr) => {
        echo($args, $out)
    };
    ($args:expr) => {
        echo::echo($args, io::stdout())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let mut output = Vec::new();
        let args = vec!["hello,", "world!"];
        echo!(args, &mut output).expect("Command failed!");

        let output = String::from_utf8(output).expect("Failed to convert to UTF-8!");
        let expected_value = "hello, world!";

        assert_eq!(output, expected_value);
    }
}
