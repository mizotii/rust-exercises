pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // implements the Draw trait, but doesn't specify a type
}

// we don't use trait bounds on purpose, otherwise our screen instance could only have a list of
// components all of the same type
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {} // will differ from other types that implement the Draw trait
}