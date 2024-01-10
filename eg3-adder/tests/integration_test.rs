mod common;

use eg3_adder;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, eg3_adder::add(2, 2));
}
