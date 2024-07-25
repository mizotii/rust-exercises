use std::ops::Deref;

struct MyBox<T>(T); // tuple struct defined over T containing T

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = Box::new(x); // points to a copied value of x (on the heap)

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // demonstrates deref coercion, which in this case turns &MyBox<String> into &String into &str
    let m = MyBox::new(String::from("rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("hello, {name}!");
}