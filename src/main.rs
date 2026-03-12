mod help;
mod request;

// TODO: Add json parsing
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help::print_help();
        return Err("Err".into());
    }
    if args[1] == "-h" || args[1] == "--help" {
        help::print_help();
        return Err("Err".into());
    }
    match request::req(&args[1], "en") {
        Ok(response) => {
            let json = String::from_utf8_lossy(&response);
            std::println!("Success! Result: {}", json);
        }
        Err(e) => std::println!("Error: {}", e),
    }
    Ok(())
}
