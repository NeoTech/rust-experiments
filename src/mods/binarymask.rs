pub fn frommask(args: &[String]) -> Vec<u32>{
    if args.len() < 1 {
        eprintln!("Usage: <integer>");
        return [0].to_vec();
    }
    /* Function that convers a number to a array of 1 and 0 */
    println!("Converting {} to binary", args[0]);
    let mask = args[0].parse::<i32>().unwrap();
    let a = format!("{:b}", mask)
        .chars()
        .map(|c| c.to_digit(10).unwrap()
        ).collect::<Vec<u32>>();
    /* Adds padding to Vector until it represents a 32 bit number */
    let mut b = a.clone();
    while b.len() < 32 {
        b.insert(0, 0);
    }
    println!("Converting {} to array of 1 and 0", mask);
    println!("{:?}", b);
    return b;
}

pub fn tomask(args: &[String]) -> u32 {
    if args.len() < 1 {
        eprintln!("Usage: <array of 1 and 0>");
        return 0;
    }
    /* Checks if argument contains anything else then 1 0 or , */
    if args[0].chars().any(|c| c != '0' && c != '1' && c != ',') {
        eprintln!("Usage: <array of 1 and 0>");
        return 0;
    }
    /* Function that convers a array of 1 and 0 to a number */
    /* ie 0,1,0 */
    let a = args[0].split(",")
        .map(|s| s.parse::<u32>()
        .unwrap())
        .collect::<Vec<u32>>();
    let mask = u32::from_str_radix(&a.iter()
            .map(|&i| i
            .to_string())
        .collect::<String>(), 2)
        .unwrap();
    /* Return value and print it */
    println!("Converting {:?} to integer", a);
    println!("{}", mask);
    return mask;
}


#[test]
fn test_tomask() {
    // Test that tomask returns 2 for the input "0,1,0"
    let args = vec!["0,1,0".to_string()];
    let result = tomask(&args);
    assert_eq!(result, 2);
}

#[test]
fn test_frommask() {
    // Test that frommask returns 0,1,0 for the input "2"
    let args = vec!["2".to_string()];
    let result = frommask(&args);
    assert_eq!(result, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
}