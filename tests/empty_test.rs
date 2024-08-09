use magical::hello_world;

#[test]
fn empty_test() {
    assert_eq!(
        (),
        hello_world()
    )
}
