use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("i ate a salad for lunch today");
    assert_eq!("", post.content());

    post.add_text(" and it was delicious");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("i ate a salad for lunch today and it was delicious", post.content());
}