use std::io::{self, Write};

fn emit_prompt() {
    print!("temp> ");
    io::stdout().flush().unwrap();
} // end emit_prompt

/*fn to_celsius(temp: f64) -> f64 {
    0.0
}

fn to_fahrenheit(temp: f64) -> f64 {
    0.0
}

fn to_kelvin(temp: f64) -> f64 {
    0.0
}
*/
fn main() {
    println!("Temperature Converter");
    println!("please enter a temperature followed by a letter for the units.");
    println!("Examples: 23f, 12.6c, 42.0k (Fahrenheit, Celsius, Kelvin)");

    loop {
        emit_prompt();

        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let value = value.trim();

        // allow a graceful exit from the program.
        if value == "quit" {
            break;
        } else if value == "exit" {
            break;
        }

        println!("You gave me: {}", value);
    }
}
