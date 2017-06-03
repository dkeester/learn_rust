/*
 * A commandline based four function calculator.
 */
use std::io::{self, Write};

fn main() {
    println!("A simple four function calculator");
    println!();
    println!("Type \"quit\" or \"exit\" to quit.");
    println!("Operators are: +, -, *, and /");
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

        if exp.contains('+') {
            //println!("Addition!");
            let v: Vec<&str> = exp.split('+').collect();

            let (x, y) = (v[0].parse::<i64>().unwrap(),
                        v[1].parse::<i64>().unwrap());

            println!("Answer: {}", add(x, y));
        } else if exp.contains('-') {
            //println!("Subtraction");
            let v: Vec<&str> = exp.split('-').collect();

            let (x, y) = (v[0].parse::<i64>().unwrap(),
                        v[1].parse::<i64>().unwrap());

            println!("Answer: {}", sub(x, y));
        } else if exp.contains('*') {
            //println!("Multiplication");
            let v: Vec<&str> = exp.split('*').collect();

            let (x, y) = (v[0].parse::<i64>().unwrap(),
                        v[1].parse::<i64>().unwrap());

            println!("Answer: {}", mul(x, y));
        } else if exp.contains('/') {
            //println!("Division");
            let v: Vec<&str> = exp.split('/').collect();

            let (x, y) = (v[0].parse::<i64>().unwrap(),
                        v[1].parse::<i64>().unwrap());

            println!("Answer: {}", div(x, y));
        } else {
            println!("Error! Try again.");
        }
    } // end loop
} // end main

fn emit_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
} // end emit_prompt

fn add(x: i64, y: i64) -> i64 {
    x+y
}

fn sub(x: i64, y: i64) -> i64 {
    x-y
}

fn mul(x: i64, y: i64) -> i64 {
    x*y
}

fn div(x: i64, y: i64) -> i64 {
    if y == 0 {
        println!("Division by zero! Answer undefined!");
        0
    } else {
        x/y
    }
}
