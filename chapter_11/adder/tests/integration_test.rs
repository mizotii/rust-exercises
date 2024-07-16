// only runs if all unit tests pass (unles you specify with cargo test --test integration_test)

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}