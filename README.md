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

## Adding new samples

All samples are in their source format in [`samples/`] directory. If you add a new sample,
make sure you run `cargo clean` before rebuilding/rerunning again.

[samples/]: https://github.com/kubkon/wasmtime-bench/tree/master/samples

## Some results

All results are after re-running each binary 1000 times. For `wasmtime`, we invoke the binary
with `--disable-cache` and `-O` flags.

A note of caution, in all presented results, the dispersion is expressed in terms of the
standard deviation which is probably not accurate but should give at least an idea of
how wildly the values differed between each other.

### Fibonnacci
Source file: [samples/fib.rs]

|          |                   |
| -------- | ----------------- |
| Native   | 30.65 +/- 3.50 ms |
| Wasmtime | 94.81 +/- 7.28 ms |

[samples/fib.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/fib.rs

### Print to screen in a loop
Source file: [samples/print.rs]

|          |                   |
| -------- | ----------------- |
| Native   | 9.10 +/- 0.75 ms  |
| Wasmtime | 62.74 +/- 6.01 ms |

[samples/print.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/print.rs

### Print "Hello world!" to screen
Source file: [samples/hello.rs]

|          |                   |
| -------- | ----------------- |
| Native   | 1.97 +/- 0.44 ms  |
| Wasmtime | 46.54 +/- 7.74 ms |

[samples/hello.rs]: https://github.com/kubkon/wasmtime-bench/tree/master/samples/hello.rs
