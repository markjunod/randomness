# Random Numbers (and Booleans)

A library for generating three of Rust's four scalar data types: integers, floating-point numbers, and booleans.

This library is more focused on the psuedo-random number generation algorithms than on providing higher-level APIs,
e.g. shuffling an array of objects.  For that we suggest using (TODO create this crate), which uses this library.

Currently the following PRNG algorithms are implemented and exposed publicly:

* Mersenne Twister (TODO add link)
* Middle Square Weyl Sequence (TODO add link)
* Xorshift+ (TODO add link)
* Xoshiro** (TODO add link)
* More to come...

All except Middle Square Weyl Sequence implement the random 64-bit version of the algorithm.  The Middle Square Weyl
Sequence implementation produces a random 32-bit number.

## Prerequisites

## How to Use

Add

```toml
[dependencies]
random_numbers=0.1.0
```

to your `Cargo.toml` file.

### Versions

The latest version is `0.1.0`, but `0.2.0` is under active development.  We use the usual `major.minor.micro` semantic
versioning system, so everything prior to `1.0.0` will be backwards compatible.  Any bug fixes will bump the micro
version.

Version `1.0.0` is not under active development right now.  Once a couple of initial minor versions have been released
we'll evaluate the state of the library and determine what is needed in order to make it "production ready".  This
may or may not include breaking changes.

### Generating Random Numbers (and Booleans)

Two public traits are provided: `RandomNumber` and `RandomNumberInit`.
