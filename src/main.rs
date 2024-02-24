mod echo;
mod cat;
mod ls;
mod find;
mod grep;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    let choice = &args[1];

    match choice.as_str() {
        "echo" => echo::echo(args.iter().skip(2).collect()),
        "cat" => cat::cat(args.iter().skip(2).collect()),
        "ls" => ls::ls(match args.len() {
            2 => ".",
            _ => &args[2]
        }),
        "find" => find::find(&args[2], &args[3]),
        "grep" => grep::grep(&args[2], &args[3]),
        x => println!("Invalid choice: {}", x)
    }
}
