use core::panic;
use std::io;

fn fib(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    fib(num - 1) + fib(num - 2)
}

fn main() {
    let mut fib_n = String::new();
    println!("Which nth Fibonacci number would you like to get?");
    io::stdin()
        .read_line(&mut fib_n)
        .expect("Failed to read line");
    let fib_n: u32 = match fib_n.trim().parse() {
        Ok(num) => fib(num),
        Err(_) => panic!("Enter a valid number"),
    };
    println!("The nth fibonacci number is: {fib_n}");
}
