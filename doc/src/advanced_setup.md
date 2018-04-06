When you setup LALRPOP, you create a `build.rs` file that looks something
like this:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

This `process_root()` call simply applies the default configuration:
so it will transform `.lalrpop` files into `.rs` files *in-place* (in
your `src` directory), and it will only do so if the `.lalrpop` file
has actually changed. But you can also use the
[`Configuration`][config] struct to get more detailed control.

[config]: https://docs.rs/lalrpop/*/lalrpop/struct.Configuration.html

For example, to **force** the use of colors in the output (ignoring
the TTY settings), you might make your `build.rs` file look like so:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .process_current_dir();
}
```
