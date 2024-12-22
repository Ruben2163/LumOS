fn parse_command(command: &str) -> &str {
    match command {
        "hello" => "Hello, User!",
        "exit" => "Exiting...",
        _ => "Unknown command",
    }
}

fn main() {
    let command = "hello"; // In real life, you would get this from user input
    let response = parse_command(command);
    println!("{}", response);
}