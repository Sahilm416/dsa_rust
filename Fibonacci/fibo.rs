// Print the nth Fibonacci number.

fn main() {
    let n: u64 = 4; // Change this to the desired number.
    let fib = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, fib);
}

fn fibonacci(num: u64) -> u64 {
    // This is recursive method (i like this one more)
    if num <= 1 {
        return num;
    }
    return fibonacci(num - 1) + fibonacci(num - 2);

    // This is iterative method
    // if num <= 1 {
    //     return num;
    // }

    // let mut a = 0;
    // let mut b = 1;
    // let mut result = 0;

    // for _ in 2..=num {
    //     result = a + b;
    //     a = b;
    //     b = result;
    // }
    // result
}
