//! # Types used in this library

pub enum Lang {
    EN,
    DE,
}

/// Defines a greeting object.
///
/// # Examples
///
/// ```
/// use minigreet::{Greeting, Lang};
/// let greeting = Greeting::new(Lang::DE);
/// ```
pub struct Greeting {
    pub prefix: String,
}

impl Greeting {
    /// Create a new instance of Greeting
    pub fn new(lang: Lang) -> Self {
        match lang {
            Lang::DE => Self {
                prefix: String::from("Hallo"),
            },
            _ => Self {
                prefix: String::from("Hello"),
            },
        }
    }

    // Return a greeting string
    pub fn greet(&self, name: &str) -> String {
        let str: String = self.prefix.to_owned();
        str + " " + name + "!!!"
    }
}
