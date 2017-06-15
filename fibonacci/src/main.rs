fn main() {
    let mut res = Vec::new();
    fibonacci(0,1, &mut res);

    println!("{:?}", res);
}

// Generate the Fibonacci sequence using the <u64> type, saving the results to a Vector.
// We need a mutable binding to a mutable reference for the Vector.
fn fibonacci(prev: u64, curr: u64, mut results: &mut Vec<u64>) {
    // store the 'prev' before it gets lost.
    results.push(prev);

    // calculate 'next' checking for overflow.
    if let Some(next) = curr.checked_add(prev) {
        // If there is a 'next' value, continue to generate the sequence.
        fibonacci(curr, next, &mut results);
    } else {
        // Make sure the last 'curr' value is in the result.
        // We are stopping due to overflowing u64.
        results.push(curr);
    }
}