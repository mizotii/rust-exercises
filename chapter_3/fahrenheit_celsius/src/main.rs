use std::io;

fn main() {
    // intro
    println!("F -> C calculator");

    // input
    loop {
        println!("please input a valid temperature in F.");
    
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
    
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = (input - 32.0) * (5.0 / 9.0);
    
        println!("the temperature: {input}, in fahrenheit, is equal to the temperature: {celsius}, in celsius");
        break;
    }
}
