#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    let mut count = 0;
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {state:?}!"),
            _ => count + 1,
        }
    }

    // or

    if let Coin::Quarter(state) = coin {
        println!("state quarter from {state:?}!");
    } else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
