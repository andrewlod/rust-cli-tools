use std::error::Error;

pub fn echo(args: Vec<&String>) -> Result<(), Box<dyn Error>> {
    let args_size = args.len();

    if args_size < 1 {
        return Err("Echo: at least 1 argument must be provided!")?;
    }
    
    for (i, arg) in args.iter().enumerate() {
        if i < args_size-1 {
            print!("{} ", arg);
            continue;
        }

        print!("{}", arg);
    }

    Ok(())
}