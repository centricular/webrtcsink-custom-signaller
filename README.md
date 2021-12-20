# webrtcsink custom signaller example

This is a simple example to demonstrate how to use webrtcsink with a custom
signaller implementation.

Run with:

``` shell
cargo run
```

The expected output is a not-implemented panic:

``` shell
thread 'async-std/runtime' panicked at 'not implemented', src/signaller/imp.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Simply implement the methods in [src/signaller/imp.rs] and you should be good
to go!
