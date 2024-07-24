use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("mariners win the world series!"),
        location: String::from("Seattle, WA, USA"),
        author: String::from("lookoutlanding"),
        content: String::from(
            "the seattle mariners once again are the best team in the MLB.",
        ),
    };

    println!("new article available! {}", article.summarize());
}