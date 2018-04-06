extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .emit_comments(true)
        .use_cargo_dir_conventions()
        .process_current_dir()
        .unwrap();
}
