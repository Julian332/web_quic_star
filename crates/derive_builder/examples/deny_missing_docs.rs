//! Some people may have `#![deny(missing_docs)]` in their crate.
//!
//! NOTE: This can only be tested in examples, but not integration tests.
#![deny(missing_docs)]

use derive_builder::WebApiGen;

/// Traditional form of communication.
#[derive(Debug, WebApiGen)]
#[builder(setter(into))]
pub struct Letter {
    /// Be creative.
    pub message: String,
}

fn main() {
    let x = LetterBuilder::default()
        .message("Hello World!")
        .build()
        .unwrap();
    println!("{}", x.message);
}
