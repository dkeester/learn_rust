/*
 * A commandline based four function calculator.
 */
use std::io::{self, Write};

fn main() {
    println!("A simple four function calculator");
    println!("Type \"quit\" or \"exit\" to quit.");
    println!();

    // process input in a loop indefinitely.
    loop {
        emit_prompt();

        let mut exp = String::new();

        io::stdin().read_line(&mut exp)
            .expect("Failed to read line");
        let exp = exp.trim();

        // allow a graceful exit from the program.
        if exp == "quit" { // '==' can do string comparison
            break;
        } else if exp == "exit" {
            break;
        }

        println!("expression: {}", exp);
    } // end loop
} // end main

fn emit_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
} // end emit_prompt
