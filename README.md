[![CI](https://github.com/zsh-rs/zsh-sys/actions/workflows/CI.yml/badge.svg)](https://github.com/zsh-rs/zsh-sys/actions/workflows/CI.yml)

# `zsh-sys`

This crate provides low-level FFI bindings to the ZSH API. It allows users to interact with ZSH internals and create ZSH modules in Rust. This crate is intended for advanced users who want to have more control over their ZSH modules and are comfortable working with unsafe code.


## Quick Start
Add `zsh-sys` as your dependency in Cargo.toml

```sh
cargo add zsh-sys
```

## Usage

Raw bindings to zsh's core headers are re-exported at the crate root:

```rust
use zsh::{Module, Builtin};
```

Bindings for zsh's bundled modules are split behind feature flags and exposed under namespaced submodules (`boot`, `setup`, `features`, `enables`, `cleanup`, `finish`):

```rust
// requires the `zle` feature
use zsh::zle::complist;

unsafe { complist::setup(module) };
```

Most users should prefer the higher-level [`zsh-module`](https://github.com/zsh-rs/zsh-module) crate, which wraps these bindings in a safe API.

## Features

- `zle` — bindings for `Src/Zle` (types, functions, and the following modules: `zutil`, `compctl`, `complist`, `computil`, `zleparameter`)
- `builtins` — bindings for `Src/Builtins` (`rlimits`, `sched`)
- `modules` — bindings for `Src/Modules` (`datetime`, `langinfo`, `parameter`, `termcap`, `terminfo`, `zutil`)
- `all` *(default)* — enables all of the above

#### Credits

<sup>
This crate is the fork of <a href="https://github.com/Diegovsky/zsh-module-rs"><code>zsh-module-rs</code></a>
and I appreciate the efforts of <a href="https://github.com/Diegovsky">@Diegovsky</a>
and other contributors.
</sup>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
