mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(5, hello::add(3, 2));
}
