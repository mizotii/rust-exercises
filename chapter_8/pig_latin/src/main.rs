use is_vowel::IsRomanceVowel;
use std::{error, io};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    loop {
        println!("text input:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let words = parse_input(&input);

        let text_tl = pig_latinize(words);

        for word in text_tl {
            println!("{word} ");
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut words = Vec::new();
    for word in input.split_whitespace() {
        words.push(word);
    }
    words
}

fn pig_latinize(words: Vec<&str>) -> Vec<String> {
    let mut pig_latin_tl = Vec::new();
        for word in words {
            let mut word_tl = String::new();
            let mut iter_c = word.chars().next();
            match iter_c {
                Some(first) => 
                    if !first.is_romance_vowel() {
                        let mut remainder = word.chars();
                        remainder.next();
                        remainder.as_str();
                        word_tl = format!("{remainder:?}{first}ay")
                    } else {
                        word_tl = format!("{word}hay");
                    },
                _ => (),
            }
            pig_latin_tl.push(word_tl);
        }
    pig_latin_tl
}