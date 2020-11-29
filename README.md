# Randomness Library

A library designed to make it as easy and intuitive as possible to inject randomness into your projects and algorithms.

It is NOT READY FOR PRODUCTION.  The [rand crate](https://crates.io/crates/rand) is a much bigger project, worked on by many fantastic Rust developers,
with much more functionality, and should be used instead.

That said, the library is (or at least, should be) functional.  Error handling may be missing or incomplete, and documentation will almost definitely
be lacking, but testing should be in place prior to new features getting merged.  That means it should be unlikely a breaking change is introduced, but I
can't promise it won't happen prior to version 1.0.0.  Also note development is slow, since it's just me, and this is just a side-project.

The goals of this project are:

* Learn Rust by creating safe, stable code (i.e. no `unsafe` usage, no nightly-only functionality)
* Learn about pseudo-random number generation and bit-manipulation
* Be as easy to use as possible
* Be fast when it doesn't conflict with the three previous goals

When it comes to speed, according to benchmarks using [criterion](https://crates.io/crates/criterion), this library currently beats the
[rand](https://crates.io/crates/rand) crate when it comes to generating primitive values.  However, as mentioned, this library isn't safe for production
use yet and may slow down as features are added, bugs are found, and error handling is hardened.

## Prerequisites

If you're using this crate, it is assumed you know about Rust, have it installed, and know about [the book](https://doc.rust-lang.org/book/).  Nothing else
is required to start using this library.

## How to Use

Add

```toml
[dependencies]
randomness = "0.1.0"
```

to your `Cargo.toml` file.

### Versions

The latest version is `0.1.0`.

I will try to create a changelog once the library itself is more stable.
