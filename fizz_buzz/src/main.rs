// A quick implementation of the fizz/buzz game
use std::string::String;

fn main() {
    let mut output = String::new();

    for x in 1..100 {
        if x % 3 == 0 {
            output.push_str("fizz");
        } else if x % 5 == 0 {
            output.push_str("buzz");
        } else {
            output = format!("{}", x);
        }

        println!("{}", output);

        output = "".to_string();
    }
}
