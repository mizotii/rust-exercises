fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using your favorite color, {color}, as the background.");
    } else if is_tuesday {
        println!("tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the background color.");
        } else {
            println!("using orange as the background color.");
        }
    } else {
        println!("using blue as the background color");
    }
}
