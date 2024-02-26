use std::{error::Error, io};

#[macro_use]
mod cat;
#[macro_use]
mod echo;
#[macro_use]
mod find;
#[macro_use]
mod grep;
#[macro_use]
mod ls;

fn invalid_option<S: AsRef<str>>(option: S) -> Result<(), Box<dyn Error>> {
    let message = &format!("Invalid option: {}", option.as_ref());
    Err(message.as_str())?
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    let choice = &args[1];

    let result = match choice.as_str() {
        "echo" => echo!(args.iter().skip(2).collect()),
        "cat" => cat!(args.iter().skip(2).collect()),
        "ls" => ls!(match args.len() {
            2 => ".",
            _ => &args[2],
        }),
        "find" => find!(&args[2], &args[3]),
        "grep" => grep!(&args[2], &args[3]),
        x => invalid_option(x),
    };

    match result {
        Err(e) => println!("The following error has occurred: {}", e),
        Ok(_v) => println!("Execution successful!"),
    }
}
