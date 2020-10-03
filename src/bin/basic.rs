extern crate idioma;
use idioma::*;
use colored::Colorize;

/// This function demonstrates use of the library.
/// Run this using `cargo run --bin basic`. Execution will exit with error - don't panic.
fn main() {
    success("Yay, you actually managed to compile this!").display();
    info("This is just a demo of what idioma can do.").display();
    custom("custom".blue().bold())("This is a custom label. You can make one too!").display();
    warning("This program will shut down with error very soon!").display();
    error("Time to say bye-bye...").exit_with();
}
