use minigreet::{Greeting, Lang};

#[test]
fn greet_english() {
    let greeting = Greeting::new(Lang::EN);
    assert_eq!("Hello John!!!", greeting.greet("John").as_str());
}

#[test]
fn greet_german() {
    let greeting = Greeting::new(Lang::DE);
    assert_eq!("Hallo John!!!", greeting.greet("John").as_str());
}
