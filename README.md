# GEDCOM

`gedcom` is a simple application that takes a directory containing one or more 
GEDCOM files as input and outputs one more corresponding JSON files to the same
directory containing data compatible with the FindMyPast relation API.

## Pre-Requisites

In order to run `gedcom` you will need to install Rust, the easiest way is to go
to [the rustup site](https://rustup.rs/) and follow the instructions for your
operating system. This project currently requires a nightly version of Rust to 
be installed in order to use the 
[DrainFilter iterator API](https://github.com/rust-lang/rust/issues/43244), you 
should therefore also run `rustup toolchain install nightly`. Once you have 
installed Rust (and the nightly toolchain) you will also need to install
[`grcov`](https://github.com/mozilla/grcov) in order to generate a test coverage
report. You can do this by running `cargo install grcov`.

## Compiling and Running the Application

Rust is a compiled language, accordingly you must first compile `gedcom` before
using it. To compile a development build of `gedcom` you can use the command
`cargo build`, to compile an optimised build for benchmarking you should instead
run `cargo build --release`.

You can then run the compiled binary in one of two ways, either way you need to
provide an argument using the `-d` flag. The `-d` flag expects an argument
consisting of the path to a directory containing GEDCOM input files. The `-d`
flag also specifies the path to write the JSON output files which will be named
identically to the corresponding input file but with a `.json` extension.

### Option 1

You can run the binary using `cargo` in development mode using
`cargo run -- -d <path/to/gedcom/directory` or in release mode using
`cargo run --release -- -d <path/to/gedcom/directory>`.

### Option 2

You can run the development binary directly using
`target/debug/gedcom -d <path/to/gedcom/directory>` or the release binary
directly using `target/release/gedcom -d <path/to/gedcom/directory>`.

For the purposes of benchmarking, the release binary should be first compiled 
and then run using Option 2 above to avoid any overhead incurred by running 
through `cargo`.

## Running the Unit Tests and Generating a Coverage Report

Tests live alongside the code in [most](#things-to-improve) source files. They 
can be found at the end of the source code in a separate `tests` module. The 
`tests` module is preceded by a `#[cfg(test)]` directive that ensures these 
modules are _not_ compiled into optimised release builds.

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

### Example Micro-Benchmark Output

An initial benchmark run may produce output similar to the folllowing:

```shell
gedcom_to_relation_json ONE_NODE                                                                            
                        time:   [119.79 us 120.22 us 120.72 us]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

gedcom_to_relation_json THREE_NODE                                                                            
                        time:   [237.35 us 238.28 us 239.43 us]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

gedcom_to_relation_json SIBLING                                                                            
                        time:   [282.29 us 282.95 us 283.68 us]
Found 15 outliers among 100 measurements (15.00%)
  11 (11.00%) high mild
  4 (4.00%) high severe
```

A subsequent benchmark run (in this case following the marking of a function as
inline-able to the compiler) may produce output similar to the following:

```shell
gedcom_to_relation_json ONE_NODE                                                                            
                        time:   [103.02 us 103.26 us 103.51 us]
                        change: [-16.984% -16.013% -15.082%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe

gedcom_to_relation_json THREE_NODE                                                                            
                        time:   [201.16 us 201.52 us 201.90 us]
                        change: [-18.292% -17.407% -16.474%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

gedcom_to_relation_json SIBLING                                                                            
                        time:   [243.63 us 244.05 us 244.52 us]
                        change: [-15.348% -14.551% -13.796%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe
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

## Things to Improve

The obvious thing to improve would be to round out test coverage, especially in
the `models` module and its submodules. 

The current `Birth` type could probably become a more generic `Fact` type. 
Alternatively, to minimise the number of `Option<T>` fields on any potential
`Fact` type, a `Fact` enumeration could be created where a `Birth` variant 
contains a `struct Birth {...}`, a `Death` variant contains a 
`struct Death {...}` and so on. In general it would be nice to complete the
types included to match the Relation API schema.

Error handling has been included in the most minimal way possible, which is to
say that most errors should result in _some_ kind of error message rather than
silent failure. Regardless, most errors will cause a failure to occur and the
application to terminate.

Missing _required_ fields, most notably `date_created` on `Person` and `Family`
types will not cause the application to exit. Instead as much of the GEDCOM as
possible is converted and output. This behaviour may or may not be desirable in
production, at the least a list of failures should be kept and output, this does
not happen currently.

Unicode support is currently limited. The application has been tested on Linux
(Ubuntu 20.04) and Mac (macOS 10.15.4) successfully using the minimal test files
included with the specification. Byte-order marks at the start of the file are
ignored, if they appears elsewhere then behaviour is undefined. Other unicode
characters, for example as part of a name, will likely cause parsing to fail, 
this has not been tested.

Beyond this I would like to investigate how to parallelise the application. Rust
provides excellent support for concurrency, for example, the
[rayon library](https://docs.rs/rayon/1.3.0/rayon/) provides very ergonomic
parallel iterator support. I investigated using a parallel iterator over files
in the target directory but to negative effect. I suspect this might be due to
all test inputs being comparatively small and so the cost of parallelisation not
being offset to a net performance gain. There are likely other places that could
be converted to use parallel iterators also.

## General Rust Resources

If you liked this project and want to know more about Rust, here are some
resources that I hope are useful:

- [The Official Site](https://www.rust-lang.org/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [The Community Forum](https://users.rust-lang.org/)
- [Rust Analyzer (IDE support)](https://rust-analyzer.github.io/)
