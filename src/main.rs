use std::env;
mod mods;
use mods::math::{add, sub, mandelbrot};
use mods::login::{login};
use mods::highlighter::{_highlight_countries};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: client <command> [<args>]");
        eprint!("Commands: greet, add, subtract, login, highlight_countries, ncurses(linux)");
        return;
    }

    let command = &args[1];
    let command_args = &args[2..];

    match command.as_str() {
        "greet" => greet(command_args),
        "add" => add(command_args),
        "sub" => sub(command_args),
        "login" => login(command_args),
        "mandelbrot" => mandelbrot(command_args),
        #[cfg(target_os = "linux")]
        "ncurses" => ncurses(command_args),
        "highlight_countries" => _highlight_countries(command_args),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn greet(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: greet <name>");
        return;
    }

    let name = &args[0];
    println!("Hello, {}!", name);
}