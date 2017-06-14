// Open a file, or read a file on stdin,
// and print it to stdout with line numbers
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}
