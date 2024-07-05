mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting; // re-export, allows for public use of hosting module

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path

    front_of_house::hosting::add_to_waitlist(); // relative path

    hosting::add_to_waitlist(); // makes use of the use keyword

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("rye"); // order breakfast in summer with rye toast
    meal.toast = String::from("wheat"); // change our mind about what break we'd like
    println!("i'd like {} toast please", meal.toast);

    // note that we cannot modify the seasonal fruit, it is private
}