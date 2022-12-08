extern crate serde_json;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use serde_json::json;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];

    let file = fs::File::open(input_file)?;
    let reader = BufReader::new(file);

    let mut output = String::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(",").collect();
        let id = parts[0];
        let name = parts[1];
        let price = parts[2];
        let description = parts[3];
        let _local = parts[4];

        let json = json!({
            id: {
                "Name": name,
                "Price": price,
                "Description": {
                    "Local": description
                }
            }
        });
        output.push_str(&json.to_string());
        output.push('\n');
    }

    io::stdout().write_all(output.as_bytes())?;

    Ok(())
}