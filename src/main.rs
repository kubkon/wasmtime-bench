use std::fs;
use std::process::{Command, Stdio};
use std::time::SystemTime;
use structopt::StructOpt;

/// Simple wasmtime vs native benchmarks.
#[derive(StructOpt)]
struct Opt {
    /// Number of repetitions (defaults to 100).
    #[structopt(short, long)]
    n: Option<u64>,
}

fn main() {
    let opt = Opt::from_args();
    let samples_dir = env!("OUT_DIR");
    for sample in fs::read_dir(samples_dir).expect("dir with compiled samples exists") {
        let sample = sample.expect("valid entry");
        let path = sample.path();
        let file_name = path
            .file_name()
            .expect("a file")
            .to_str()
            .expect("valid Unicode");
        let ext = path.extension().map(|x| x.to_str().expect("valid Unicode"));
        let path = path.to_str().expect("valid Unicode");
        let (m, s) = if let Some(ext) = ext {
            if ext == "wasm" {
                println!("Timing '{}'", file_name);
                timeit(wasmtime_cmd(&path), &opt.n)
            } else {
                panic!("unexpected file extension {}", ext);
            }
        } else {
            println!("Timing '{}'", file_name);
            timeit(native_cmd(&path), &opt.n)
        };
        println!("\t{} +/- {} ms", m, s);
        println!();
    }
}

fn wasmtime_cmd(path: &str) -> Command {
    let mut cmd = Command::new("wasmtime");
    cmd.arg("--disable-cache")
        .arg("-O")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    cmd
}

fn native_cmd(path: &str) -> Command {
    let mut cmd = Command::new(path);
    cmd.stdout(Stdio::null()).stderr(Stdio::inherit());
    cmd
}

fn timeit(mut cmd: Command, repetitions: &Option<u64>) -> (f64, f64) {
    let mut timings = vec![];
    let repetitions = repetitions.unwrap_or(100);
    for _ in 0..repetitions {
        let t1 = SystemTime::now();
        let _output = cmd.status().expect("command successful");
        let duration = t1.elapsed().expect("monotonic time");
        timings.push(duration.as_secs_f64() * 1000.);
    }
    let m = mean(&timings);
    let s = stdev(&timings);
    (m, s)
}

fn mean(xs: &[f64]) -> f64 {
    let sum: f64 = xs.iter().sum();
    let len = xs.len() as f64;
    sum / len
}

fn stdev(xs: &[f64]) -> f64 {
    let mean = mean(xs);
    let sum: f64 = xs.iter().map(|x| (x - mean) * (x - mean)).sum();
    let len = xs.len() as f64;
    (sum / (len - 1.0)).sqrt()
}
