pub fn frommask(args: &[String]) {
    if args.len() < 1 {
        eprintln!("Usage: <integer>");
        return;
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
    println!("{:?}", b);
}

pub fn tomask(args: &[String]) {
    /* Checks if argument contains anything else then 1 0 or , */
    if args[0].chars().any(|c| c != '0' && c != '1' && c != ',') {
        eprintln!("Usage: <array of 1 and 0>");
        return;
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
    println!("{:?}", mask);
}