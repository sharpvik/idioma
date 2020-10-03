extern crate colored;

use std::process::exit;
use colored::*;


pub fn success(message: String) {
    println!("{}{} {}", "success".green().bold(), ":".bold(), message);
}

pub fn warning(message: String) {
    println!("{}{} {}", "warning".yellow().bold(), ":".bold(), message);
}

pub fn info(message: String) {
    println!("{}{} {}", "info".yellow().bold(), ":".bold(), message);
}

pub fn error(message: String) {
    println!("{}{} {}", "error".red().bold(), ":".bold(), message);
}

pub fn exit_with(callback: fn(String), message: String) {
    callback(message);
    exit(1);
}
