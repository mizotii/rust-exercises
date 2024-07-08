use std::io;

fn main() {
    loop {
        println!("text input:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let words = parse_input(&input);


    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut words = Vec::new();
    for word in input.split_whitespace() {
        words.push(word);
    }
    words
}

fn pig_latinize(words: Vec<&str>) -> Vec<&str> {
    let mut pig_latin_tl = Vec::new();
        for word in words {

        }

    pig_latin_tl
}