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
            "A"|"a" => {
                let exec = options.get("option1").unwrap();
                exec();
            },
            "B"|"b" => {
                let exec = options.get("option2").unwrap();
                exec();
            },
            "C"|"c" => {
                let exec = options.get("option3").unwrap();
                exec();
            }
            "Q"|"q" => break,
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
    print!("{}[2J", 27 as char);
    println!("Option 1 selected");
}
fn option2() {
    print!("{}[2J", 27 as char);
    println!("Option 2 selected");
}
fn option3() {
    print!("{}[2J", 27 as char);
    println!("Option 3 selected");
}