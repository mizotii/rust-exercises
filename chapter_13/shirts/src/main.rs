use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor { // if a user has a preference, we give them that preference. if not, we simply give them our most stocked. thinking about this helps us understand why closures come in handy.
        user_preference.unwrap_or_else(|| self.most_stocked()) // || self.most_stocked() is the closure expression
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

//

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    
    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };

    let add_one_v3 = |x: u32| {
        x + 1
    };

    let add_one_v4 = |x: u32| x + 1;

    //

    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); // compiler infers the type of x and the return value to be string...
    // let n = example_closure(5); ...making this become a type error.

    //

    let list = vec![1, 2, 3];
    println!("before defining closure: {list:?}");

    let only_borrows = || println!("from closure: {list:?}");

    println!("before calling closure: {list:?}");
    only_borrows();
    println!("after calling closure: {list:?}");

    //

    let mut list2 = vec![1, 2, 3];
    println!("before defining closure: {list2:?}");

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("after calling closure: {list2:?}");

    //

    let list3 = vec![1, 2, 3];
    println!("before defining closure: {list3:?}");

    thread::spawn(move || println!("from thread: {list3:?}")) // moves list3 into the closure
        .join()
        .unwrap();

    //

    let mut list4 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list4.sort_by_key(|r| r.width); // FnMut closure -- does not move values out of its body. also is called multiple times (once for each item in the slice)
    println!("{list4:#?}");

    let mut num_sort_operations = 0;
    list4.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list4:#?}, sorted in {num_sort_operations} operations");
} 
