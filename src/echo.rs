use std::{error::Error, io};

pub fn echo(args: Vec<&String>, mut out: impl io::Write) -> Result<(), Box<dyn Error>> {
    let args_size = args.len();

    if args_size < 1 {
        Err("Echo: at least 1 argument must be provided!")?;
    }

    for (i, arg) in args.iter().enumerate() {
        if i < args_size - 1 {
            write!(&mut out, "{} ", arg)?;
            continue;
        }

        write!(&mut out, "{}", arg)?;
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
