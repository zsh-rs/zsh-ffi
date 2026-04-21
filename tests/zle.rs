#![cfg(feature = "zle")]

mod smoke_tests {
    #![allow(unused_imports)]
    
    use zsh::zle::Keymap;
    use zsh::zle::Thingy;
    use zsh::zle::Widget;
    use zsh::zle::addzlefunction;
    use zsh::zle::keymap;
    use zsh::zle::zleparameter::setup;
    use zsh::zle::thingy;
    use zsh::zle::widget;
    use zsh::zle::zlecs;
    use zsh::zle::zleline;
    use zsh::zle::zrefresh;
}

#[test]
fn zle_interface() {
    assert!(true);
}