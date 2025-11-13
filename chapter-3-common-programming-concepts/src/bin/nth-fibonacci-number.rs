// Generate the nth Fibonacci number
// The Fibonacci sequence is a sequence
// where the next term is the sum of the
// previous two terms. The first two terms
// of the Fibonacci sequence are 0 followed
// by 1.
fn main() {
    const N: i32 = 6;
    let mut previous1 = 0;
    let mut previous2 = 1;
    let mut number = 0;
    for _ in 0..N-1 {
        number = previous1 + previous2;
        previous1 = previous2;
        previous2 = number;
    }
    if N == 1 {
        println!("{previous2}")
    } else {
        println!("{number}");
    }
}
