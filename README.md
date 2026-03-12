# `zsh-sys`

This crate provides low-level FFI bindings to the ZSH API. It allows users to interact with ZSH internals and create ZSH modules in Rust. This crate is intended for advanced users who want to have more control over their ZSH modules and are comfortable working with unsafe code.


## Usage

<!-- TODO: -->


## Roadmap

<!-- template
- ✅ 
- 🚧 
- 📋 
 -->

### Source

- ➕ Use `git` submodules to vendor ZSH C headers
    - Where do we source the headers from, is there a mirror we can leverage?
- ➕ Update .gitignore: config.h

### Build
- ➕ Run `./configure` to target user's system correctly