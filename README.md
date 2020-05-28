# GEDCOM

`gedcom` is a simple application that takes a GEDCOM file as input and outputs a
JSON file containing data compatible with the FindMyPast relation API.

## Pre-Requisites

In order to run `gedcom` you will need to install Rust, the easiest way is to go
to [the rustup site](https://rustup.rs/) and follow the instructions for your
operating system. Once you have installed Rust you will also need to install
[`grcov`](https://github.com/mozilla/grcov) in order to generate a test coverage
report. If you have first installed Rust you can do this by running
`cargo install grcov`.

## Compiling and Running the Application

Rust is a compiled language, accordingly you must first compile `gedcom` before
using it. To compile a development build of `gedcom` you can use the command
`cargo build`, to compile an optimised build for benchmarking you should instead
run `cargo build --release`.

You can then run the compiled binary in one of two ways, either way you need to
provide two arguments using the `-i` and `-o` flags. The `-i` flag expects an
argument consisting of the path to the GEDCOM input file. The `-o` flag expects
an argument consisting of the path to write the JSON output file.

### Option 1

You can run the binary using `cargo` in development mode using
`cargo run -- -i <path/to/input.ged> -o <path/to/desired.json>` or in release
mode using
`cargo run --release -- -i <path/to/input.ged> -o <path/to/desired.json>`.

### Option 2

You can run the development binary directly using
`target/debug/gedcom -i <path/to/input.ged> -o <path/to/desired.json>` or the
release binary directly using
`target/release/gedcom -i <path/to/input.ged> -o <path/to/desired.json>`.

For the purposes of benchmarking the release binary should be first compiled and
then run using Option 2 above to avoid any overhead incurred by running through
`cargo`.

## Running the Tests and Generating a Coverage Report

Tests live alongside the code in most source files. They can be found at the end
of the source code in a separate `tests` module. The `tests` module is preceded
by a `#[cfg(test)]` directive that ensures these modules are _not_ compiled into
optimised release builds.

The tests themselves may be run using `cargo test`. A coverage report may be
generated by running the `generate-coverage-report.sh` script. This script uses
`cargo clean` to first remove _all previously compiled binaries_. This step is
necessary to re-compile the development binary with specific options needed to
allow for coverage generation to be completed. You _will_ need to re-compile any
release builds before running benchmarks as outlined above.

## Running the Micro-Benchmarks

The [Criterion library](https://bheisler.github.io/criterion.rs/book/index.html)
was used in development to ensure performance regressions did not occur. To this
end a small benchmark can be found in `benches/small_benchmark.rs`. Criterion
acts as a test harness, but also produces output in the below format that
allows regressions to be tracked between changes with a degree of statistical
significance.

### Example Output

An initial benchmark run may produce output similar to the folllowing:

```shell
gedcom_to_relation_json ONE_NODE
                        time:   [289.65 us 300.53 us 312.37 us]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

gedcom_to_relation_json THREE_NODE
                        time:   [480.55 us 496.24 us 514.78 us]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

gedcom_to_relation_json SIBLING
                        time:   [578.72 us 597.32 us 619.63 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

A subsequent benchmark run (in this case following the marking of a function as
inline-able to the compiler) may produce output similar to the following:

```shell
gedcom_to_relation_json ONE_NODE
                        time:   [256.40 us 257.08 us 257.85 us]
                        change: [-38.275% -29.482% -19.934%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

gedcom_to_relation_json THREE_NODE
                        time:   [458.71 us 459.72 us 461.01 us]
                        change: [-7.8661% -5.0149% -2.1319%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high severe

gedcom_to_relation_json SIBLING
                        time:   [595.04 us 665.34 us 747.79 us]
                        change: [-10.509% -4.6092% +2.5256%] (p = 0.16 > 0.05)
                        No change in performance detected.
Found 21 outliers among 100 measurements (21.00%)
  4 (4.00%) high mild
  17 (17.00%) high severe
```

In addition to the command-line output provided as an overview, a HTML report is
generated automatically by Criterion and can be view by going to
`file:///</path/to/project/root>/target/criterion/report/index.html` in your
browser of choice.

The examples above were run on an Early 2015 Macbook Pro with an 2.9GHz Intel i5
CPU, 8GB 1867MHz DDR3 RAM and macOS v10.15.4. Your results may vary dependent on
machine specification.

## Project Structure

The project is split into two parts, or in Rust parlance, crates. The
application crate, consisting only of the file `src/main.rs`, handles execution
of the program. The library crate consisting of all other files in the `src`
directory, with an entrypoint in `src/lib.rs` provides all underlying GEDCOM
specific functionality. The library crate is in turn largely organised into two
submodules, `models` and `parser`.

### Dependencies and their Uses

Here is a brief overview of the third-party crates used in this library. These
crates are declared in the manifest file `Cargo.toml` under the `[dependencies]`
section.

|Crate|Description|Official Site / Documentation URL|
|---|---|---|
|`chrono`|A Date & Time library|[Documentation](https://docs.rs/chrono/0.4.11/chrono/)|
|`clap`|Command-Line Argument Parser|[Official Site](https://clap.rs/)|
|`lazy_static`|A macro for declaring lazily evaluated statics|[Documentation](https://docs.rs/lazy_static/1.4.0/lazy_static/)|
|`nom`|Nom is a Parser-combinator library|[Documentation](https://docs.rs/nom/6.0.0-alpha1/nom/index.html)|
|`regex`|Regular expressions library|[Documentation](https://docs.rs/regex/1.3.7/regex/)|
|`serde`|Serialization and deserialization framework|[Official Site](https://serde.rs/)|
|`serde_json`|JSON serialization and deserialization using `serde`|[Documentation](https://docs.serde.rs/serde_json/)|
|`serde_repr`|Serialization and deserialization of enumerated types using user-specified representation|[Documentation](https://docs.rs/serde_repr/0.1.5/serde_repr/)|
