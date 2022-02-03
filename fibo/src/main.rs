fn main() {
    println!("{}!", fib(9));
    println!("{}!", fib(10));
    println!("{}!", fib(1));
}

fn fib(n: i32) -> i32 {
  if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
}