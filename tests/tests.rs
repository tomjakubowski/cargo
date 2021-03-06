#![feature(macro_rules)]
#![feature(phase)]

extern crate term;
extern crate cargo;
extern crate hamcrest;

#[phase(plugin, link)]
extern crate log;

mod support;
macro_rules! test(
    ($name:ident $expr:expr) => (
        #[test]
        fn $name() {
            ::support::paths::setup();
            setup();
            $expr;
        }
    )
)

mod test_cargo_compile;
mod test_cargo_compile_git_deps;
mod test_cargo_compile_path_deps;
mod test_cargo_test;
mod test_shell;
