//! How to use this program:
//!
//! Get random generated password containing letters and numbers that is 20 characters length:
//! ep
//!
//! Mixing words for password:
//! ep words
//!
//! Substitute letters and symbols with other letters symbols similar to them and mix words:
//! ep -s words
//!
//! Generate password that has 10 characters in length with optional words to use
//! ep -l 10 [words]
//!
//! Generate password including special symbols like: "$%^&@()[]{}\/!?|":;',.<>+_*~`:
//! ep -S [words]
//!
//! You can also combine options. For random password that is 15 characters length and has special
//! symbols:
//! ep -Sl 15
//! If you want to include some words in your password:
//! ep -lS 15 [words]

fn main() {
    easy_pass::run();
}
