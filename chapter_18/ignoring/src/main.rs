fn foo(_: i32, y: i32) {
    println!("this code only uses the y param: {y}");
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    foo(3, 4);

    //

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    //

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {first}, {third}, {fifth}")
        }
    }

    //

    let _x = 5; // no warning
    let y = 10; // warning

    let s = Some(String::from("hello!"));

    if let Some(_) = s { // will not write s to _, unlike if we were to use Some(_s)
        println!("found a string!");
    }

    println!("{s:?}");

    //

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first, .., last) => {
            println!("some numbers: {first}, {last}")
        }
    }

    //

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("the number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    //

    
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("matched, n = {n}"), // n == y is not a pattern and thus does not shadow the y in the outer scope
        _ => println!("default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    //

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("found an id in another range"),
        Message::Hello { id } => println!("found some other id: {id}"),
    }
}
