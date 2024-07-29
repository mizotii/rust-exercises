struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x: 0, y: 0 } => println!("at the origin"),
        Point { x, y: 0 } => println!("on the x axis at {x}"),
        Point { x: 0, y } => println!("on the y axis at {y}"),
        Point { x, y } => println!("on neither axis at ({x}, {y})"),
    }

    //

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change the color to red {r}, green {g}, and blue {b}")
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change the color to hue {h}, saturation {s}, and value {v}")
        }
        _ => (),
    }

    //

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
