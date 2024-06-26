use std::io;

fn main() {
    println!("please input a fahrenheit tempurature.");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to read line.")

    let temperature: u32 = temperature
        .trim()
        .parse()
        .expect("please type a number!")
}
