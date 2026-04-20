#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod bindings {
    use crate::base::Module;
    include!(concat!(env!("OUT_DIR"), "/builtins.rs"));
}

module!(rlimits, use bindings);
module!(sched, use bindings);
