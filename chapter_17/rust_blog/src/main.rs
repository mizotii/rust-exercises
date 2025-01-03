use rust_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("i ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("i ate a salad for lunch today", post.content());
}