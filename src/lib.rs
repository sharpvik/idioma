#![crate_name = "idioma"]
#![crate_type = "lib"]

//! As Rust developers we deeply care about safety and error handling - our programs are fast and
//! reliable. However, users never make it easy for us: they misunderstand instructions and break
//! things. When we catch them doing something they shouldn't be doing, we let them know (usually)
//! with an error message.
//!
//! Every command line tool prints handy messages to `stdout` from time to time, and to do so,
//! requires a function or two. I noticed that whenever I start a new project I tend to copy the
//! `util.rs` that contains those display functions from my last project. That is simply no good.
//!
//! It means that my error messages
//! - differ in style (since I regularly alter code in that util file);
//! - don't look like idiomatic Rust messages;
//! - require that `copy + paste` operation for every new project.
//!
//! And I strongly believe that I am not alone in this. Take a look at [this code][1] by
//! [brain-lang]:
//!
//! ```
//! macro_rules! exit_with_error(
//!     ($($arg:tt)*) => { {
//!         use std::process;
//!         eprintln!($($arg)*);
//!         process::exit(1);
//!     } }
//! );
//! ```
//!
//! [1]: https://github.com/brain-lang/brainfuck/blob/master/src/bin/brainfuck.rs#L21
//! [brain-lang]: https://github.com/brain-lang/
//!
//! As you can see, they wrote this macro right next to the `main` function and it is the same
//! problem that I have with my util file.
//!
//! The `idioma` library solves all these problems forever. Here's how.
//!
//! In your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! idioma = "*"
//! ```
//!
//! Include in any Rust file.
//!
//! ```
//! extern crate idioma;
//! ```
//!
//! Use within a function.
//!
//! ```
//! fn foo(i: i32) {
//!     if i != 42 {
//!         idioma::exit_with(error, "Your taste is appalling.")
//!     }
//! }
//! ```

extern crate colored;

use std::{
    fmt::Display,
    process::exit,
};
use colored::*;


/// Allows you to create and print messages with custom labels.
///
/// # Example
///
/// ```
/// use idioma::*;
/// use colored::*;
/// custom("custom".blue().bold())("This is a custom label. You can make one too!");
/// ```
pub fn custom<I>(label: ColoredString) -> impl Fn(I) where I: Display {
    move |message| println!("{}{} {}", label, ":".bold(), message)
}

/// Displays a success message.
///
/// # Example
///
/// ```
/// use idioma::*;
/// success("A man of honour must always strive for a greater success in life.");
/// ```
pub fn success<I>(message: I) where I: Display {
    custom("success".green().bold())(message);
}

/// Displays a warning.
///
/// # Example
///
/// ```
/// use idioma::*;
/// warning("Very soon, you will run out of water.");
/// ```
pub fn warning<I>(message: I) where I: Display {
    custom("warning".yellow().bold())(message);
}

/// Displays a neutral info message.
///
/// # Example
///
/// ```
/// use idioma::*;
/// info("I came here to write some code and kiss some pretty ladies and, as you can see, \
///       I'm done with the code.");
/// ```
pub fn info<I>(message: I) where I: Display {
    custom("info".purple().bold())(message);
}

/// Displays a bright-red error message that draws attention.
///
/// # Example
///
/// ```
/// use idioma::*;
/// error("You were not supposed to mess with me!");
/// ```
pub fn error<I>(message: I) where I: Display {
    custom("error".red().bold())(message);
}

/// Calls `callback` with `message` and terminates the program via `std::process::exit(1)`.
///
/// # Examples
///
/// ```
/// use idioma::*;
/// exit_with(error, "You were not supposed to mess with me!");
/// ```
///
/// You can even combine `custom` and `exit_with` to produce some real nice stuff.
///
/// ```
/// use idioma::*;
/// use colored::*;
/// exit_with(custom("lol".cyan().bold()), "Did you expect something serious here? LMAO XD");
/// ```
pub fn exit_with<C, I>(callback: C, message: I) where C: Fn(I), I: Display {
    callback(message);
    exit(1);
}
