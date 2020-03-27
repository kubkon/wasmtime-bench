fn main() {
    std::process::exit(run() as i32);
}

#[no_mangle]
pub extern "C" fn run() -> u32 {
    fib(35)
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
