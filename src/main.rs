use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if is_ai_command(&args) {
        respond_with_hi();
    } else {
        print_usage();
    }
}

fn is_ai_command(args: &[String]) -> bool {
    args.len() > 1 && args[1] == "ai"
}

fn respond_with_hi() {
    println!("hi");
}

fn print_usage() {
    eprintln!("Usage: my_cli ai <input>");
}
