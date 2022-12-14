use std::io::*;
use std::collections::HashMap;

fn main() {
    // Create a map of menu options to functions
    let mut options: HashMap<&str, fn()> = HashMap::new();
    options.insert("option1", option1);
    options.insert("option2", option2);
    options.insert("option3", option3);

    
    // Print the menu and handle user input
    loop {
        print_menu(options.clone());
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        /* A match statement that uses input */
        match input.trim() {
            "A" => {
                let exec = options.get("option1").unwrap();
                exec();
            },
            "B" => {
                let exec = options.get("option1").unwrap();
                exec();
            },
            "C" => {
                let exec = options.get("option1").unwrap();
                exec();
            }
            "Q" => break,
            _ => println!("Invalid input"),
        }
        
    }
}

fn print_menu(options: HashMap<&str, fn()>) {
    let letters = ['A', 'B', 'C', 'Q'];
    for(i, (key,_)) in options.iter().enumerate() {
        println!("{}: {}", letters[i], key);
    }
    println!("Q: Quit");
    println!("-----");
    print!("> ");
    stdout().flush().unwrap();
}

fn option1() {
    println!("Option 1 selected");
}
fn option2() {
    println!("Option 2 selected");
}
fn option3() {
    println!("Option 3 selected");
}