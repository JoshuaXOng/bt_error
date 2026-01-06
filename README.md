# BT Error - Backtrace Error

Adds a type `WithBacktrace` that boxes an inner type. Used to capture the
`Backtrace` when the inner type is created. Crate also has utilities to
propagate the `Backtrace` up the stack.

`Backtrace` capturing does exist in other error crates (e.g.,
`anyhow`, `thiserror`) but the UX, API and capabilities are slightly different.

For example, to setup backtraces whilst maintaining conversions that you'd
otherwise do on say a `Result<_, B>` in a function that returns `Result<_, A>`. Where:
- `A impl Error` and `B impl Error`
- `impl From<B> for A`

`anyhow` nicely propagates the `Backtrace` for you but `A` would be limited
to it's own type where the inner type is opague, a trait object.
Converting, `?`'ing, a `thiserror` with a `Backtrace` to an `anyhow`, the error
is now opague. The actual, lowest-level `Backtrace` might get lost.
Converting from `anyhow` to `thiserror` requires you to implement the
`Backtrace` logic for each custom error; same for converting between
different `thiserror`s.

# Examples

Assuming `From` and `Error` implementations already exist, the setup to allow
conversions whilst propagating the `Backtrace`.
```rust
use bt_error::{define_with_backtrace, Backtrace};

define_with_backtrace!();

#[derive(Backtrace)]
struct ErrorA;

#[derive(Backtrace)]
struct ErrorB;

fn b_to_bt_a() -> Result<(), WithBacktrace<ErrorA>> {
    let e = ErrorB;
    Err(e)?;
    Ok(())
}

fn bt_b_to_bt_a() -> Result<(), WithBacktrace<ErrorA>> {
    let e = WithBacktrace::new(ErrorB);
    Err(e)?;
    Ok(())
}
```

Explicitly declaring conversion implementation of struct to
`WithBacktrace` with inner type an `Error` trait object.
This is already implemented when using the `Backtrace` derive macro.
```rust
use std::error::Error;
use bt_error::define_with_backtrace;

define_with_backtrace!();
define_to_dyn!(std::io::Error);

fn external_to_bt_dyn() -> Result<(), WithBacktrace<Box<dyn Error>>> {
    let e = std::io::Error::other("test");
    Err(e)?;
    Ok(())
}
```

# License

This project is licensed under the [MIT license](LICENSE).
