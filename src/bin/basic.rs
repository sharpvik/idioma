extern crate idioma;
use colored::Colorize;
use idioma::*;

/// This function demonstrates use of the library.
/// Run this using `cargo run --bin basic`. Execution will exit with error - don't panic.
fn main() {
    success("Yay, you actually managed to compile this!").print();
    info("This is just a demo of what idioma can do.").print();
    custom("custom".blue().bold())("This is a custom label. You can make one too!").print();
    warning("This program will shut down with error very soon!").print();
    debug("But you shouldn't worry, it's normal.").print();
    error("Time to say bye-bye...").exit(1);
}
