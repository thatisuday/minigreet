//! # MiniGreet: A minimal greeting libary
//! This is a simple test library for creating small greetings
//! in various languages and it's pretty basic.

// [define modules]
mod types;

// [re-exports]
pub use types::{Greeting, Lang};

/// Returns a greeting in a specific language.
///
/// # Examples
///
/// ```
/// use minigreet::Lang;
///
/// let name = "John";
/// let greeting = minigreet::greet(name, Lang::DE);
///
/// assert_eq!(greeting.as_str(), "Hallo John!!!");
/// ````
pub fn greet(name: &str, lang: Lang) -> String {
    let greeting = Greeting::new(lang);
    greeting.greet(name)
}
