use std::io;

fn main() {
    println!("Please enter number!");

    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    calculate_fibonacci(num);
}

fn calculate_fibonacci(num: u32) {
    let mut n1: i32 = 0;
    let mut n2: i32 = 1;
    let mut next_term: i32;

    for _ in (1..num+1) {
        println!("Number is {}", n1);
        next_term = n1 + n2;
        n1 = n2;
        n2 = next_term;
    }
}