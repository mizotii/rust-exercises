use core::fmt::Debug;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

//

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) { // accepts any type that implements Summary (i.e. NewsArticle, Tweet)
    println!("breaking news! {}", item.summarize());
}

// actually looks like this (trait bound):
// pub fn notify<T: Summary>(item: &T) {
//     println!("breaking news! {}", item.summarize());    
// }

pub fn different_types(item1: &impl Summary, item2: &impl Summary) {}

pub fn force_same_type<T: Summary>(item1: &T, item2: &T) {}

pub fn notify_display(item: &(impl Summary + Display)) {}

pub fn notify_display_trait_bound<T: Summary + Display>(item: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 0 }

fn returns_summarizable() -> impl Summary { // returns a value of some type that implements a trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}