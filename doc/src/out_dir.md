One common request is to have LALRPOP generate its files into Cargo's
**output directory**, rather than storing them 'in-place' within the
module tree. This can be done through the following configuration:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .process();
}
```

In addition, because the modules are generated out of the `src`
directory, for each `foo.lalrpop` file you can't simply have `mod
foo;` in your source.  Instead, you must put the following in the
parent module:

```rust
include!(concat!(env!("OUT_DIR"), "/path/to/foo.rs"));
```

Here the `path/to/foo.rs` should be relative to the `src` directory.
