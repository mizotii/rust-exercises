use std::io;

fn main() {
    println!("welcome to the fibonacci calculator.");

    let result = loop {
        println!("please enter a positive integer:");

        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("failed to read input.");
    
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break fib_calc(n)
    };

    println!("the result is: {result}");
}

fn fib_calc(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib_calc(n - 1) + fib_calc(n - 2)
    }
}