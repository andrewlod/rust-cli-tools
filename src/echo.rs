pub fn echo(args: Vec<&String>) {
    let args_size = args.len();

    if args_size < 1 {
        println!("Echo: at least 1 argument must be provided!");
        return;
    }
    
    for (i, arg) in args.iter().enumerate() {
        if i < args_size-1 {
            print!("{} ", arg);
            continue;
        }

        print!("{}", arg);
    }
}