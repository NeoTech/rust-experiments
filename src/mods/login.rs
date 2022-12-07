pub fn login(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: login <username> <password>");
        return;
    }

    let username = &args[0];
    let password = &args[1];
    println!("Logging in as {} with password {}", username, password);
}

#[test]
fn login_test() {
    let args = vec!["username".to_string(), "password".to_string()];
    login(&args);
}
