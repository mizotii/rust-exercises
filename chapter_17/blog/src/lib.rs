const NOMINATIONS: i32 = 2;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    nominations: i32,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            nominations: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // consumes the current state, returns a new state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // take sets state to None temporarily so that we can get ownership of the state value and to not let Post use the old state value
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // should change the states, which are Box<Self> types
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve<'a>(self: Box<Self>, post: &'a mut Post) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    // should return content
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    //
    fn add_text<'a>(self: Box<Self>, post: &'a mut Post, text: &str) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve<'a>(self: Box<Self>, post: &'a mut Post) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(self: Box<Self>, post: &'a mut Post, text: &str) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve<'a>(self: Box<Self>, post: &'a mut Post) -> Box<dyn State> {
        post.nominations += 1;
        if post.nominations == NOMINATIONS {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_text<'a>(self: Box<Self>, post: &'a mut Post, text: &str) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve<'a>(self: Box<Self>, post: &'a mut Post) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(self: Box<Self>, post: &'a mut Post, text: &str) -> Box<dyn State> {
        self
    }
}