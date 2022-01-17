use simple_test;

#[test]
fn test_add() {
    assert_eq!(4,simple_test::add(2,2));
}

#[test]
fn test_rest() {
    assert_eq!(0,simple_test::rest(2,2));
}

#[test]
fn test_div() {
    assert_eq!(1,simple_test::div(2,2));
}

#[test]
fn test_multi() {
    assert_eq!(4,simple_test::multi(2,2));
}