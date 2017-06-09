// A quick implementation of the fizz/buzz game

fn main() {
    for x in 1..100 {
        let mut output = format!("{} ", x);

        if x % 3 == 0 {
            output = output + "fizz ";
        }

        if x % 5 == 0 {
            output = output + "buzz ";
        }

        println!("{}", output);
    }
}
