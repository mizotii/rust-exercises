use std::{collections::HashMap, hash::Hash};

fn main() {
    // vectors

    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("the third element is {third}");

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third element."),
    }

    // iterating

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }

    // enum for multiple types

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];

    // utf-8 strings

    let mut s = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    let s2 = "initial contents".to_string();

    let s2 = String::from("initial contents");

    let mut s3 = String::from("foo");
    s3.push_str("bar");

    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5); // does not take ownership of s5, therefore we can println it
    println!("s5 is {s5}");

    let mut s6 = String::from("lo");
    s.push('l');

    // concatenation with + or format!

    let s7 = String::from("hello, ");
    let s8 = String::from("world!");
    let s9 = s7 + &s8; // + actually uses the add method, which takes in a String (and takes ownership of it) and a string slice

    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");

    let s14 = format!("{s10}-{s11}-{s12}"); // call does not take ownership of its params here

    // iterating over strings

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // hash maps

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // overwriting a value

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);

    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    println!("{scores:?}");

    // updating a value based on the old value

    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map2:?}");
}
