extern crate idioma;
use idioma::*;
use std::fs::File;
use std::io::Read;

/// This demonstrates the use of `idioma::into` and `idioma::exit_if_error`.
/// Note that `unwrap` here is completely safe since we would have exited on error.
/// Run this using `cargo run --bin result`. Execution will exit with error - don't panic.
fn main() {
    let data = exit_if_error(good("src/bin/result.rs")).unwrap();
    println!("{}", data);
    let data = exit_if_error(good("non-existent.txt")).unwrap();
    println!("{}", data);
}

/// Here you can see how we employ `idioma::into` to wrap errors from other modules like `io::Error`
/// into our own `idioma::Error` type and use `?` to control flow with comfort.
fn good(path: &str) -> Result<String, Error> {
    let mut data = String::new();
    let mut file = idioma::into(File::open(path))?;
    idioma::into(file.read_to_string(&mut data))?;
    Ok(data)
}
