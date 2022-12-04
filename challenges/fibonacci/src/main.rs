fn main() {
    let fibonacci_n: u32 = 10;
    println!(
        "Fibonacci number at N = {} is {}",
        fibonacci_n,
        fibonacci_seeker(fibonacci_n)
    );
}

fn fibonacci_seeker(n: u32) -> u32 {
    let mut counter: u32 = 3;
    let mut fibonacci_i: u32 = 0;
    let mut fibonacci_j: u32 = 1;
    let mut fibonacci_k;
    let result = loop {
        fibonacci_k = fibonacci_i + fibonacci_j;
        if counter == n {
            break fibonacci_k;
        } 
        counter += 1;
        if n == 1 {
            break fibonacci_i;
        }
        if n == 2 {
            break fibonacci_j;
        }
        fibonacci_i = fibonacci_j;
        fibonacci_j = fibonacci_k;
    };
    return result;
}
