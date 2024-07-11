use is_vowel::IsRomanceVowel;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let output = translate(&input);

    for word in output {
        println!("{word} ");
    }
}

fn translate(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    for mut word in input.split_whitespace() {
        let mut iter = word.chars();
        let first = iter.next();
        match first {
            Some(first) => 
                if first.is_romance_vowel() {
                    words.push(format!("{word}hay"));
                } else {
                    let remainder = iter.as_str();
                    words.push(format!("{remainder}{first}ay"));
                },
            _ => (),
        }
    }
    words
}