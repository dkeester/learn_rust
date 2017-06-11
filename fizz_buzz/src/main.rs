// A quick implementation of the fizz/buzz game

fn main() {
    for x in 1..101 {
        let mut output = format!("{} ", x);

        if x % 3 == 0 {
            output += "fizz ";
        }

        if x % 5 == 0 {
            output += "buzz ";
        }

        println!("{}", output);
    }
}
