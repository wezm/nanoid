//! A tiny, secure, URL-friendly, unique string ID generator
//!
//! **Safe.** It uses cryptographically strong random APIs
//! and guarantees a proper distribution of symbols.
//!
//! **Compact.** It uses a larger alphabet than UUID (`A-Za-z0-9_~`)
//! and has a similar number of unique IDs in just 21 symbols instead of 36.
//!
//! ```toml
//! [dependencies]
//! nanoid = "0.1.2"
//! ```
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn main() {
//!    let id = nanoid::simple(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
//! }
//! ```
//!
//! ## Usage
//!
//! ### Simple
//!
//! The main module uses URL-friendly symbols (`A-Za-z0-9_~`) and returns an ID
//! with 21 characters.
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn main() {
//!    let id = nanoid::simple(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
//! }
//! ```
//!
//! Symbols `-,.()` are not encoded in the URL. If used at the end of a link
//! they could be identified as a punctuation symbol.
//!
//! ### Custom length
//!
//! If you want to reduce ID length (and increase collisions probability),
//! you can pass the length as an argument generate function:
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn main() {
//!    let id = nanoid::generate(10); //=> "IRFa~VaY2b"
//! }
//! ```
//!
//! ### Custom Alphabet or Length
//!
//! If you want to change the ID's alphabet or length
//! you can use the low-level `custom` module.
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn main() {
//!     let alphabet: [char; 16] = [
//!         '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
//!     ];
//!
//!    let id = nanoid::custom(10, &alphabet); //=> "4f90d13a42"
//! }
//! ```
//!
//! Alphabet must contain 256 symbols or less.
//! Otherwise, the generator will not be secure.
//!
//! ### Custom Random Bytes Generator
//!
//! You can replace the default safe random generator using the `complex` module.
//! For instance, to use a seed-based generator.
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn randomBytes () -> u32 { /* ... */ }
//!
//! fn main() {
//!     fn random (size: usize) -> Vec<u32> {
//!         let mut bytes: Vec<u32> = vec![0; size];
//!
//!         for i in 0..size {
//!             bytes[i] = randomBytes();
//!         }
//!
//!         bytes
//!     }
//!
//!     nanoid::complex(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
//! }
//! ```
//!
//! `random` function must accept the array size and return an vector
//! with random numbers.
//!
//! If you want to use the same URL-friendly symbols with `format`,
//! you can get the default alphabet from the `url` module:
//!
//! ```rust
//! extern crate nanoid;
//!
//! fn random (size: usize) -> Vec<u32> { /* ... */ }
//!
//! fn main() {
//!     nanoid::complex(10, nanoid::alphabet::SAFE, random); //=> "93ce_Ltuub"
//! }
//! ```
//!

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid")]

extern crate rand;

mod random;
mod generator;
pub mod alphabet;

pub fn complex(size: usize, alphabet: &[char], random: fn(usize) -> Vec<u32>) -> String {
    let x = alphabet.len();

    // if (x == 2^n)
    let generator = if (x != 0) && ((x & (x - 1)) == 0) { generator::fast } else { generator::universal };

    generator(random, alphabet, size)
}

pub fn custom(size: usize, alphabet: &[char]) -> String {
    complex(size, alphabet, random::standart)
}

pub fn generate(size: usize) -> String {
    custom(size, &alphabet::SAFE)
}

pub fn simple() -> String {
    generate(21)
}
