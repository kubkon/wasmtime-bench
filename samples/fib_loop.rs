fn main() {
    std::process::exit(fib(43) as i32)
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b += tmp;
    }
    b
}
