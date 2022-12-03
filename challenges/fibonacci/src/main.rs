fn main() {
    let fibonacci_n: i32 = 2;
    println!(
        "Fibonacci number at N = {} is {}",
        fibonacci_n,
        fibonacci_seeker(fibonacci_n)
    );
}

fn fibonacci_seeker(n: i32) -> u32 {
    let n: i32 = n - 2;
    let mut counter: i32 = 0;
    let mut fibonacci_i: u32 = 0;
    let mut fibonacci_j: u32 = 1;
    let mut fibonacci_k: u32 = 0;
    let result = loop {
        counter += 1;
        if counter >= n {
            break fibonacci_k;
        }
        fibonacci_k = fibonacci_i + fibonacci_j;
        fibonacci_i = fibonacci_j;
        fibonacci_j = fibonacci_k;
    };
    return result;
}
