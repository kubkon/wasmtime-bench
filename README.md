# wasmtime-bench

Simple benchmarks of [wasmtime] vs native.

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

All samples are in their source format in [`samples/`] directory. If you add a new sample,
make sure you run `cargo clean` before rebuilding/rerunning again.

[samples/]: https://github.com/kubkon/wasmtime-bench/tree/master/samples

## Some results

All results are after re-running each binary 200 times. For `wasmtime`, we invoke the binary
with `--disable-cache` and `-O` flags.

A note of caution, in all presented results, the dispersion is expressed in terms of the
standard deviation which is probably not accurate but should give at least an idea of
how wildly the values differed between each other.

### Fibonacci with recursive calls
Source file: [samples/fib_func.rs]

|          |                      |
| -------- | -------------------- |
| Native   | 1309.12 +/-  8.55 ms |
| Wasmtime | 2649.92 +/- 36.71 ms |

[samples/fib_func.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/fib_func.rs

### Fibonacci with loops
Source file: [samples/fib_loop.rs]

|          |                   |
| -------- | ----------------- |
| Native   | 4.93 +/- 42.52 ms |
| Wasmtime | 38.02 +/- 3.32 ms |

[samples/fib_loop.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/fib_loop.rs
