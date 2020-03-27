# wasmtime-bench

Simple benchmarks of [wasmtime] (JIT) vs native.

[wasmtime]: https://wasmtime.dev

## Building and running

You need `wasm32-wasi` target installed and you need to have `wasmtime` in your `PATH`.

Run

```
rustup target add wasm32-wasi
```

to install `wasm32-wasi` target.

In order to get the latest precompiled `wasmtime` binary for your platform, head over to
this [link].

[link]: https://github.com/bytecodealliance/wasmtime/releases

Afterwards, to build and run the benchmarks

```
cargo run --release -- --n 100
```

where `--n` is the number of repetitions.

## Adding new samples

All samples are in their source format in [samples/] directory. If you add a new sample,
make sure you run `cargo clean` before rebuilding/rerunning again.

[samples/]: https://github.com/kubkon/wasmtime-bench/tree/master/samples

## Some results

All results are after re-running each binary 100 times. For `wasmtime`, we invoke the binary
with `--disable-cache` and `-O` flags.

A note of caution, in all presented results, the dispersion is expressed in terms of the
standard deviation which is probably not accurate but should give at least an idea of
how wildly the values differed between each other.

### Fibonacci with recursive calls
Source file: [samples/fib_func.rs]

|          |                     |
| -------- | ------------------- |
| Native   | 1004.88 +/- 4.03 ms |
| Wasmtime | 1976.96 +/- 8.83 ms |

[samples/fib_func.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/fib_func.rs

### Fibonacci with loops
Source file: [samples/fib_loop.rs]

|          |                   |
| -------- | ----------------- |
| Native   | 0.44 +/- 0.03 ms  |
| Wasmtime | 13.88 +/- 0.62 ms |

[samples/fib_loop.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/fib_loop.rs

### Mandelbrot
Adapted from [The Computer Language Benchmarks Game].

Source file: [samples/mandelbrot.rs]

|          |                      |
| -------- | -------------------- |
| Native   | 5891.91 +/- 14.53 ms |
| Wasmtime | 9217.38 +/- 28.94 ms |

[The Computer Language Benchmarks Game]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/mandelbrot-rust-6.html
[samples/mandelbrot.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/mandelbrot.rs
