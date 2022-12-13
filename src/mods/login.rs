struct User {
    username: String,
    password: String,
}

impl User {
    fn login(&self) -> bool {
        /* Check username and passwords value */
        if self.username == "admin" && self.password == "password" {
            return true;
        } else {
            return false;
        }
    }
}


/* A function that is public that uses the login struct to login a user. */
pub fn login(args: &[String]) -> bool {
    if args.len() < 2 {
        eprintln!("Usage: <username> <password>");
        return false;
    }
    let user = User {
        username: args[0].clone(),
        password: args[1].clone(),
    };
    return user.login();
}

#[test]
fn test_login() {
    // Test that login returns true for the input "admin admin"
    let user = User {
        username: "admin".to_string(),
        password: "password".to_string(),
    };
    let result = user.login();
    assert_eq!(result, true);
}