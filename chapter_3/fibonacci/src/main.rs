use std::io;

fn main() {
    println!("Fibonacci Sequence");
    println!("Type the nth number of the sequence you wanna know");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Error reading the line");

    let nth_number: i32 = nth_number.trim().parse().expect("Not a number");

    let fibonacci_number = fibonacci(nth_number);

    println!("Your fibonacci number is: {fibonacci_number}");
}

fn fibonacci(n: i32) -> i32 {
    let mut result: i32 = 0;

    if n == 1 {
        result = 0;
        return result
    } else if n == 2 {
        result = 1;
        return result
    }

    let mut  first = 0;
    let mut second = 1;

    for number in (2..n) {
        result = first + second;
        first = second;
        second = result;
    }

    result
}
