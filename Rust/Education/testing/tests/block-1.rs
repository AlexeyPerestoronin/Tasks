use testing::*;

#[test]
#[should_panic]
fn case_1() -> () {
    match mul(5, 5) {
        Ok(v) => assert_eq!(v, 25),
        Err(_) => panic!("Multiplication was wrong!"),
    }
}

#[test]
fn case_2() -> Result<(), String> {
    assert_eq!(mul(4, 5)?, 20);
    Ok(())
}
