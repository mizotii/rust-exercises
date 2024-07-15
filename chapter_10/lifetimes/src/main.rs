use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // for some lifetime 'a, the function takes two parameters,
                                                    // both of which are string slices that live at least as
                                                    // long as lifetime 'a
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//

struct ImportantExcerpt<'a> {
    part: &'a str, // an instance of ImportantExcerpt can't outlive the reference it holds in its part field
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {announcement}");
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T, // can be filled in by any type which implements the Display trait...
) -> &'a str // x and y have the same lifetime as the return type
where
    T: Display, // ... as is specified here
{
    println!("announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;

    let r = &x;

    println!("r: {r}");

    // generic lifetimes in functions

    let string1 = String::from("looooooooong string");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("the longest string is {result}");
    
    // lifetime annotations in struct definitions

    let novel = String::from("call me ishmael. some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
