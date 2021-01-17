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
//! - require that `COPY + PASTE` operation for every new project.
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
//! use idioma::*;
//! fn foo(i: i32) {
//!     if i != 42 {
//!         error("Your taste is appalling.").exit(1);
//!     }
//! }
//! ```

extern crate colored;

use colored::*;
use std::{
    fmt::{self, Display},
    process,
};

/// `Text` is the main type that gets thrown around between functions and methods. It is basically a
/// `String`, but it had to be made into a separate `struct` so that it would be possible to `impl`
/// some things for it.
#[derive(Debug)]
pub struct Text {
    text: String,
}

/// `Error` type is an alias of the `Text` type that we use to denote errors specifically. The fact
/// that this is an alias also means that it has access to all the methods that `Text` has.
pub type Error = Text;

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Text {
    /// Returns `Text` by constructing it from a given `label` and `message`.
    ///
    /// # Examples
    ///
    /// ```
    /// use idioma::*;
    /// use colored::*;
    /// Text::make("lol".cyan().bold(), "LMAO you're so funny.");
    /// ```
    pub fn make<I>(label: ColoredString, message: I) -> Self
    where
        I: Display,
    {
        Self {
            text: format!("{}{} {}", label, ":".bold(), message),
        }
    }

    /// Displays `Text` thanks to the `std::fmt::Display` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use idioma::*;
    /// warning("This message is going to be printed out immediately!").print();
    /// ```
    pub fn print(&self) {
        println!("{}", self)
    }

    /// Displays `message` and terminates the program via `std::process::exit`. Please note that
    /// this function returns `Text` back in case we need to please the type checker. See
    /// `exit_if_error` function for an example of that.
    ///
    /// # Examples
    ///
    /// ```
    /// use idioma::*;
    /// error("You were not supposed to mess with me!").exit(1);
    /// ```
    ///
    /// You can even combine `custom` and `exit` to produce some real nice stuff.
    ///
    /// ```
    /// use idioma::*;
    /// use colored::*;
    /// custom("lol".cyan().bold())("Did you expect something serious here? LMAO XD").exit(1);
    /// ```
    #[allow(unreachable_code)]
    pub fn exit(self, code: i32) -> Self {
        self.print();
        process::exit(code);
        self
    }
}

/// Allows you to create and print messages with custom labels. Essentially, allows you to write
/// your own functions like `error`, `info`, etc. that we already have here.
///
/// # Example
///
/// ```
/// use idioma::*;
/// use colored::*;
/// let custom_label = custom("custom".blue().bold());
/// custom_label("This is a custom label. You can make one too!");
/// custom_label("Declare it once and reuse.");
/// ```
pub fn custom<I>(label: ColoredString) -> impl Fn(I) -> Text
where
    I: Display,
{
    move |message| Text::make(label.clone(), message)
}

/// Returns a green and shiny success message.
///
/// # Example
///
/// ```
/// use idioma::*;
/// success("A man of honour must always strive for a greater success in life.");
/// ```
pub fn success<I>(message: I) -> Text
where
    I: Display,
{
    Text::make("success".green().bold(), message)
}

/// Debug your code with style.
///
/// # Example
///
/// ```
/// use idioma::*;
/// debug("The nuclear missiles have been launched!");
/// ```
pub fn debug<D>(message: D) -> Text
where
    D: Display,
{
    Text::make("debug".blue().bold(), message)
}

/// Displays a warning.
///
/// # Example
///
/// ```
/// use idioma::*;
/// warning("Very soon, you will run out of water.");
/// ```
pub fn warning<I>(message: I) -> Text
where
    I: Display,
{
    Text::make("warning".yellow().bold(), message)
}

/// Returns a neutral info message.
///
/// # Example
///
/// ```
/// use idioma::*;
/// info("I came here to write some code and kiss some pretty ladies and, as you can see, \
///       I'm done with the code.");
/// ```
pub fn info<I>(message: I) -> Text
where
    I: Display,
{
    Text::make("info".purple().bold(), message)
}

/// Returns a bright-red error message that draws attention.
///
/// # Example
///
/// ```
/// use idioma::*;
/// error("You were not supposed to mess with me!");
/// ```
pub fn error<I>(message: I) -> Error
where
    I: Display,
{
    Text::make("error".red().bold(), message)
}

/// Use `into` to turn any `Result` type with a displayable error into `Result<O, idioma::Error>`.
/// This will allow you to use all methods and functions defined for `idioma::Error` without having
/// to explicitly wrap errors from other libraries yourself.
///
/// See an example in `src/bin/result.rs`.
pub fn into<O, E>(result: Result<O, E>) -> Result<O, Error>
where
    E: Display,
{
    match result {
        Ok(o) => Ok(o),
        Err(e) => Err(error(e)),
    }
}

/// Somethimes you get a `Result` and you want to continue execution as normal if case it's `Ok` or
/// exit if it's `Err`. This function allows you to do precisely that.
///
/// It gives you back the `Result`, but you will only ever be able to actually touch it if you
/// passed in an `Ok` because `exit_if_error` will simply terminate on `Err`. Therefore, you can do
/// something like this:
///
/// ```
/// use idioma::*;
/// let message = exit_if_error(Ok("hello world")).unwrap();
/// println!(message);
/// ```
///
/// In this case, the call to `unwrap` is totally safe. In some cases, compiler
/// will shout at you for doing that, so just give it `#[allow(unused)]`.
pub fn exit_if_error<O>(result: Result<O, Error>) -> Result<O, Error> {
    match result {
        Ok(o) => Ok(o),
        Err(e) => Err(e.exit(1)),
    }
}
