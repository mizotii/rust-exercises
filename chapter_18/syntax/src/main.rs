fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y = {y}"),
        _ => println!("default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // my guess:
    // matched, y = 5
    // at the end: x = Some(5), y = 10

    // pattern or

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // match to range

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}