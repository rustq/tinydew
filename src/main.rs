use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "-h" | "--help" => print_help(),
        "-V" | "--version" => println!("tinydew {}", env!("CARGO_PKG_VERSION")),
        "status" => {
            let state = tinydew::db::load_or_create();
            tinydew::ui::render_status(&state);
        }
        "do" => {
            if args.len() < 3 {
                eprintln!("Usage: tinydew do <action> [args]");
                process::exit(1);
            }
            let mut state = tinydew::db::load_or_create();
            let action_args: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            let result = tinydew::action::execute(&mut state, &action_args);
            println!("{}", result);
            tinydew::db::save(&state);
        }
        other => {
            eprintln!("Unknown command: {}", other);
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("tinydew - A cozy farming game");
    println!();
    println!("Usage: tinydew <COMMAND> [ARGS...]");
    println!();
    println!("Commands:");
    println!("  status    Show current game status");
    println!("  do        Execute an action (move, clear, plant, water, harvest, fish, buy, sell, sleep)");
    println!();
    println!("Options:");
    println!("  -h, --help     Display help information");
    println!("  -V, --version  Display version information");
}
