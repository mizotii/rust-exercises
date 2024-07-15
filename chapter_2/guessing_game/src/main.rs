use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "guess value must be greater than or equal to 1, got {value}"
            );
        } else if value > 100 {
            panic!(
                "guess value must be less than or equal to 100, got {value}"
            );
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("the secret number is between 1 and 100.");
            continue;
        }
    
        println!("you guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        } 
    }
}
