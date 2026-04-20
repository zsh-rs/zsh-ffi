#![cfg(feature = "modules")]

mod smoke_tests {
    #![allow(unused_imports)]

    use zsh::modules::datetime::setup as _;
    use zsh::modules::langinfo::setup as _;
    use zsh::modules::parameter::setup as _;
    use zsh::modules::termcap::setup as _;
    use zsh::modules::terminfo::setup as _;
    use zsh::modules::zutil::setup as _;
}

#[test]
fn modules_interface() {
    assert!(true);
}