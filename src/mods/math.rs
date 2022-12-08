pub fn sub(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: subtract <num1> <num2>");
        return;
    }

    let num1 = args[0].parse::<i32>().unwrap();
    let num2 = args[1].parse::<i32>().unwrap();
    println!("{} - {} = {}", num1, num2, num1 - num2);
}

pub fn add(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: add <num1> <num2>");
        return;
    }

    let num1 = args[0].parse::<i32>().unwrap();
    let num2 = args[1].parse::<i32>().unwrap();
    println!("{} + {} = {}", num1, num2, num1 + num2);
}

/* function to generate ascii mandelbrot fractal based on command_args input, without using ncurses. And with Test code */
use std::io::Write;
use std::fs::File;
use std::path::Path;

pub fn julia(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: julia <width> <height>");
        return;
    }
    let width = args[0].parse::<usize>().unwrap();
    let height = args[1].parse::<usize>().unwrap();
    let mut buffer = vec![0u8; width * height];
    let mut file = File::create(Path::new("julia.txt")).unwrap();
    let mut x = 0;
    let mut y = 0;
    while y < height {
        while x < width {
            let cx = x as f64 * 3.5 / width as f64 - 2.5;
            let cy = y as f64 * 2.0 / height as f64 - 1.0;
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut i = 0;
            while zx * zx + zy * zy < 4.0 && i < 50 {
                let xtemp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp;
                i += 1;
            }
            buffer[y * width + x] = i as u8;
            x += 1;
        }
        y += 1;
        x = 0;
    }
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let i = buffer[y * width + x];
            if i == 50 {
                file.write(b"*").unwrap();
            } else {
                file.write(b" ").unwrap();
            }
            x += 1;
        }
        file.write(b"

").unwrap();
        y += 1;
    }
}

#[test]
fn julia_test() {
    assert_eq!(julia(&vec!["40".to_string(), "40".to_string()]), ());
}

/* A function that generates a mandelbrot fractal in ascii output */

pub fn mandelbrot(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: mandelbrot <width> <height>");
        return;
    }
    let width = args[0].parse::<usize>().unwrap();
    let height = args[1].parse::<usize>().unwrap();
    let mut buffer = vec![0u8; width * height];
    let mut file = File::create(Path::new("mandelbrot.txt")).unwrap();
    let mut x = 0;
    let mut y = 0;
    while y < height {
        while x < width {
            let cx = x as f64 * 3.5 / width as f64 - 2.5;
            let cy = y as f64 * 2.0 / height as f64 - 1.0;
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut i = 0;
            while zx * zx + zy * zy < 4.0 && i < 50 {
                let xtemp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp;
                i += 1;
            }
            buffer[y * width + x] = i as u8;
            x += 1;
        }
        y += 1;
        x = 0;
    }
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let i = buffer[y * width + x];
            if i == 50 {
                file.write(b"*").unwrap();
            } else {
                file.write(b" ").unwrap();
            }
            x += 1;
        }
        file.write(b"

").unwrap();
        y += 1;
    }
}

#[test]
fn mandelbrot_test() {
    assert_eq!(mandelbrot(&vec!["80".to_string(), "80".to_string()]), ());
}

/* test for add */
#[test]
fn add_test() {
    assert_eq!(add(&vec!["1".to_string(), "2".to_string()]), ());
}

/* test for sub */
#[test]
fn sub_test() {
    assert_eq!(sub(&vec!["1".to_string(), "2".to_string()]), ());
}