use test_organization::adds_two;

#[test]
fn it_adds_two() {
    let result = adds_two(3);
    assert_eq!(result, 5);
}