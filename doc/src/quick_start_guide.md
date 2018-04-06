For getting started with LALRPOP, it's probably best if you read
[the tutorial](tutorial/index.html), which will introduce you
to the syntax of LALRPOP files and so forth. 

But if you've done this before, or you're just the impatient sort,
here is a quick 'cheat sheet' for setting up your project.  First, add
the following lines to your `Cargo.toml`:

```toml
[package]
...
build = "build.rs" # LALRPOP preprocessing

# The generated code depends on lalrpop-util.
#
# The generated tokenizer depends on the regex crate.
#
# (If you write your own tokenizer, or already have the regex
# crate, you can skip this dependency.)
[dependencies]
lalrpop-util = "0.15.1"
regex = "0.2.0"

# Add a build-time dependency on the lalrpop library:
[build-dependencies]
lalrpop = "0.15.1"
```

Next create a `build.rs` file that looks like:

```rust
extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

(If you already have a `build.rs` file, you should be able to just
call `process_root` in addition to whatever else that file is doing.)

That's it! Note that `process_root` simply uses the default settings.
If you want to configure how LALRPOP executes, see the
[advanced setup](advanced_setup.html) section.

#### Running manually

If you prefer, you can also run the `lalrpop` crate as an
executable. Simply run `cargo install lalrpop` and then you will get a
`lalrpop` binary you can execute, like so:

```
lalrpop file.lalrpop
```

This will generate `file.rs` for you. Note that it only executes if
`file.lalrpop` is newer than `file.rs`; if you'd prefer to execute
unconditionally, pass `-f` (also try `--help` for other options).

