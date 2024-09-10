# ctx_error

## Description
Ergonomic, static, contextual errors in Rust.

## Details
### What are Contextual Errors?
In a (typically larger) Rust program which is handling errors correctly, by default Rust does not 
capture the location (filename, line number and column number) where an `Error` was first emitted.

Further, a function typically has incomplete context around *why* it is accessing a resource (e.g.
opening a file, establishing a network connection, locking a resource, etc.).  A contextual error
library provides convenient techniques for attaching context to an `Error` at various (and 
multiple) levels of the call stack.

If you use dynamic errors types (`dyn core::error::Error`) in Rust, there are many crates including 
[`anyhow`](https://crates.io/crates/anyhow) that you can use for contextual error-handling.

But if you use static errors (no type erasure) for your libraries and/or applications, at least as
of the time of this writing, there are no Rust solutions available which allow you to ergonomically
(without significant repetitive boilerplate) and idiomatically (without significant changes to your
existing `Error` types) benefit from contextual errors in your code.

In significant applications, considerable engineering time can be spent deducing the context of an
emitted `Error`, not including the time to understand, fix and ship/redeploy the solution.
`ctx_error` is designed, with proper use, to provide the necessary context with the `Error`, to
increase the speed at which a bugfix can be implemented.

This library is `#[no_std]` compatible.  Compile with `cargo build --no-default-features` to build
in `#[no_std]` environments.  When compiled for `#[no_std]`, `ctx_error` will no longer support
`Backtrace`s (a `std` feature) or user context (`append_ctx()`)--user context is implemented as a
`Vec<String>` (also a `std` feature) to support user context from multiple levels of the call
stack.

## Usage
Let's imagine a project layout looking something like this:
```
src/
  +- error
  | +- init.rs
  | +- user_input.rs
  +- error.rs
  +- init.rs
  +- lib.rs
```

`error.rs` looks like:
```rust
// error.rs
pub mod init;
pub mod user_input;

use core::result::Result as CoreResult;

use thiserror::Error;

type Result<T, E = Error> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Init(#[from] init::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    UserInput(#[from] user_input::Error),
}
```

and `init.rs` contains:
```rust
// error/init.rs
use core::result::Result as CoreResult;

use thiserror::Error;

type Result<T, E = Error> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment variable {_0} not set")]
    EnvironmentVarNotSet(String),
    #[error(transparent)]
    UnreadableConfigFile(#[from] std::io::Error),
    // ...
}
```

Given the above project, `main()` might look something like:
```rust
// main.rs
use lib::{init, error::Result};

fn main() -> Result<()> {
    let config_reader = init::config_reader()?;
    let app = App::new(reader)?;
    app.run()?;
  
    Ok(())
}
```

And `config_reader()` might look like:
```rust
// init.rs
use std::io::Read;

use crate::error::init::Result;

pub fn config_reader() -> Result<impl Read> where
{
    // ...
    let config_reader = File::open(config_path)?;
    Ok(config_reader)
}
```

Above, `init()` returns an `error::init::Error`, but `main()` returns an `error::Error`.  The 
`init::Error` returned by `init()` is converted to an `Error` via the `From` impl defined within
`Error`.  But the user sees no information on where the 
`Err(Error::Init(UnreadableConfigFile(std::io::Error)))` was emitted.  In this trivial example, it
could only have come from one place.  But in a full application, a "File not found" error could be
emitted from potentially dozens, hundreds or even thousands of locations, and without additional
information, a lot of time will be spent tracking down the source of the error.

Now let's update the example with `ctx_error`.  In the update, we make the following changes to each
`Error` definiton module:
1. From your crate root folder type `cargo add ctx_error`.
To each error module:
2. Add `use ctx_error::ctx_error`.
3. Add `ctx_error!(`<Error type>`, `<optional: path to Parent Error (typically `super::Error`)`>);`
4. (Optional) Change `Result` alias' default `Error` type to `CtxError`.

`ctx_error` is designed to add minimal additional boilerplate to the `Error` definition pattern.  It
effectively creates a set of `CtxError`s which wrap the locally defined `Error` with additional
context information, without requiring each `Error` variant to carry (repetitive `Context` type
information).  By avoiding changing the definition of `Error`s, we also leave construction and usage
of those `Error`s intact.

`CtxError` implements `From` converters for the local `Error` type, enabling seamless transition
from `Error` -> corresponding `CtxError` using the `try` operator (`?`).

`CtxError` also implements `core::error::Error`, so it can be used anywhere any other 
`core::error::Error` can be used.

Updated `error.rs` (root error definition):
```rust
// error.rs
pub mod init;
pub mod user_input;

use core::result::Result as CoreResult;

// New!
use ctx_error::ctx_error;
use thiserror::Error;

//                   v-- new!
type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Init(#[from] init::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    UserInput(#[from] user_input::Error),
}

// New!
ctx_error!(Error);
```

Updated `init.rs` sub-error definition:
```rust
// error/init.rs
use core::result::Result as CoreResult;

// New!
use ctx_error::ctx_error;
use thiserror::Error;

//                   v-- new!
type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment variable {_0} not set")]
    EnvironmentVarNotSet(String),
    #[error(transparent)]
    UnreadableConfigFile(#[from] std::io::Error),
    // ...
}

// New!
ctx_error!(Error, super::Error);
```

Now running the same program gives a different result:
```
No such file or directory (os error 2)
```
becomes:
```
No such file or directory (os error 2) at src/init.rs:9:5
```

With `RUST_BACKTRACE=1 cargo run`:
```
No such file or directory (os error 2) at src/main.rs:7:5
Error Backtrace
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:116:5
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2: std::backtrace::Backtrace::create
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/backtrace.rs:331:13
... snip ...
  19: std::rt::lang_start_internal
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/rt.rs:143:20
  20: std::rt::lang_start
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/rt.rs:163:17
  21: _main
```

`ctx_error` provides `append_ctx()` method on all `core::result::Result` and all `CtxError`s, so the
`Error` can easily be associated with arbitrary user context.  Imagine `append_context()` is used in
`init::config_reader()` as follows:

```rust
// init.rs
use std::io::Read;

use crate::error::init::Result;

pub fn config_reader() -> Result<impl Read> where
{
  // ...
  //                                              v-- new!
  let config_reader = File::open(config_path).append_ctx(config_path)?;
  Ok(config_reader)
}
```

and `main()` is modified as follows:
```rust
// main.rs
use lib::error::Result;

fn main() -> Result<()> {
    //                                                     v-- new!
    let config_reader = lib::init::config_reader().append_context("Reading app config")?;
    let app = App::new(reader)?;
    app.run()?;
  
    Ok(())
}
```

Now the run output will change from:
```
No such file or directory (os error 2)
```

to (filename is exemplar)
```
No such file or directory (os error 2) at src/init.rs:9:5
Context (Display order: error origination site -> program entry point):
  1: config/my_app_config.json
  2: Reading app config
```

or, with `RUST_BACKTRACE=1 cargo run`:
```
No such file or directory (os error 2) at src/init.rs:9:5
Context (Display order: error origination site -> program entry point):
  1: config_files/my_app_config.json
  2: Reading app config
Error Backtrace
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:116:5
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2: std::backtrace::Backtrace::create
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/backtrace.rs:331:13
... snip ...
  19: std::rt::lang_start_internal
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/rt.rs:143:20
  20: std::rt::lang_start
             at /rustc/26b5599e4d6ed2b45152c60493c1788c0a27533d/library/std/src/rt.rs:163:17
  21: _main
```

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
