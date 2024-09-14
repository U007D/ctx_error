//! Ergonomic, static, contextual errors in Rust.
//!
//! ## Details
//! ### What are Contextual Errors?
//! In a (typically larger) Rust program which is handling errors correctly, by default Rust does
//! not capture the location (filename, line number and column number) where an `Error` was first
//! emitted.
//!
//! Further, a function typically has incomplete context around *why* it is accessing a resource
//! (e.g. opening a file, establishing a network connection, locking a resource, etc.).  A
//! contextual error library provides convenient techniques for attaching context to an `Error` at
//! various (and multiple) levels of the call stack.
//!
//! If you use dynamic errors types (`dyn core::error::Error`) in Rust, there are many crates
//! including [`anyhow`](https://!crates.io/crates/anyhow) that you can use for contextual error-handling.
//!
//! But if you use static errors (no type erasure) for your libraries and/or applications, at least
//! as of the time of this writing, there are no Rust solutions available which allow you to
//! ergonomically (without significant repetitive boilerplate) and idiomatically (without
//! significant changes to your existing `Error` types) benefit from contextual errors in your code.
//!
//! In significant applications, considerable engineering time can be spent deducing the context of
//! an emitted `Error`, not including the time to understand, fix and ship/redeploy the solution.
//! `ctx_error` is designed, with proper use, to provide the necessary context with the `Error`, to
//! increase the speed at which a bugfix can be implemented.
//!
//! This library is `#[no_std]` compatible.  Compile with `cargo build --no-default-features` to
//! build in `#[no_std]` environments.  When compiled for `#[no_std]`, `ctx_error` will no longer
//! support `Backtrace`s (a `std` feature) or user context (`append_ctx()`)--user context is
//! implemented as a `Vec<String>` (also a `std` feature) to support user context from multiple
//! levels of the call stack.

#![feature(error_generic_member_access)]

pub mod ctx_error;
pub mod error;
pub mod shared_const;
pub mod r#trait;
