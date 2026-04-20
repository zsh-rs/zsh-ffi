#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod zle {}

pub mod modules {}

pub mod builtins {}



#[cfg(test)]
mod binding_smoke_tests {
    #![allow(unused_imports)]

    // Base bindings
    use super::hashtable;
    use super::param;
    use super::builtin;
    use super::linkroot;
    use super::shfunc;
    use super::HashTable;
    use super::Param;
    use super::Module;
    use super::LinkList;
    use super::addbuiltins;
    use super::addhookfunc;
    use super::builtintab;

    // zle::
    use super::widget;
    use super::thingy;
    use super::keymap;
    use super::Widget;
    use super::Thingy;
    use super::Keymap;
    use super::zrefresh;
    use super::addzlefunction;
    use super::zleline;
    use super::zlecs;
    use super::setup_zshQszleparameter;

    // modules::
    use super::setup_zshQsdatetime;
    use super::setup_zshQslanginfo;
    use super::setup_zshQsparameter;
    use super::setup_zshQstermcap;
    use super::setup_zshQsterminfo;
    use super::setup_zshQszutil;

    // builtins::
    use super::setup_zshQssched;
    use super::setup_zshQsrlimits;
}