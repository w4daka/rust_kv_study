use rust_kv_study::{parse_command, Command};

fn main() {
    let input = "SET user_name rust_ace";
    match parse_command(input) {
        Ok(cmd) => println!("Parsed: {:?}", cmd),
        Err(e) => eprintln!("Error: {}", e),
    }
}
