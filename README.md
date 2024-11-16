# Ray Tracing

* [Ray Tracing in One Weekend Series](https://raytracing.github.io/)
   * [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
   * [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
   * [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

# Running

You can build the binary as outlined in [Building](#building) and then run the
generated executable as `./target/release/raytracing` or
`./target/debug/raytracing` depending on which build you used. Alternatively,
you can use the following to run the debug/development build:

```shell
cargo run
```

or, for the release/optimized build:

```shell
cargo run --release
```

# Building

To build the debug/development build:

```shell
cargo build
```

For release/optimized build:

```shell
cargo build --release
```
