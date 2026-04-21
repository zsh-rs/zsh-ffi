#![cfg(feature = "builtins")]

mod smoke_tests {
    #![allow(unused_imports)]

    use zsh::builtins::rlimits::setup as _;
    use zsh::builtins::sched::setup as _;
}

#[test]
fn builtins_interface() {
    assert!(true);
}