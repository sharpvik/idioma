extern crate idioma;
use idioma::*;

fn main() {
    success("Yay, you actually managed to compile this!".to_string());
    info("This is just a demo of what idioma can do.".to_string());
    warning("This program will shut down with error very soon!".to_string());
    exit_with(error, "Time to say good-bye...".to_string());
}

