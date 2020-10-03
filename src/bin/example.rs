extern crate idioma;
use idioma::*;
use colored::Colorize;

/// This function demonstrates use of the library.
fn main() {
    success("Yay, you actually managed to compile this!");
    info("This is just a demo of what idioma can do.");
    custom("custom".blue().bold())("This is a custom label. You can make one too!");
    warning("This program will shut down with error very soon!");
    exit_with(custom("lol".cyan().bold()),
              "Did you expect something serious here? LMAO XD");
}
