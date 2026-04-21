use std::path::{Path, PathBuf};
use std::{env, fs, io};

use zsh_src::{ZshSource, resolve};

macro_rules! file_regex {
    ($src:expr, $subdir:expr) => {
        format!(r"{}/.*?Src{}/[^/]*\.(h|mdh|epro)", $src, $subdir)
    };
}

fn main() -> io::Result<()> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let ZshSource { source, .. } = resolve()?;
    println!("cargo::rerun-if-changed={}", source.display());

    let wrapper = generate_wrapper(&source, &out_path)?;

    let src = source.display();

    let builder = bindgen::Builder::default()
        .header(wrapper.to_str().ok_or(io::Error::other("impossible"))?)
        .clang_arg(format!("-I{src}"))
        .clang_arg("-DMODULE")
        .clang_arg("-H")
        .allowlist_recursively(false)
        .derive_default(true)
        .wrap_unsafe_ops(true);

    #[cfg(feature = "zle")]
    builder
        .clone()
        .allowlist_file(file_regex!(src, "/Zle"))
        .generate()
        .map_err(io::Error::other)?
        .write_to_file(out_path.join("zle.rs"))?;

    #[cfg(feature = "builtins")]
    builder
        .clone()
        .allowlist_file(file_regex!(src, "/Builtins"))
        .generate()
        .map_err(io::Error::other)?
        .write_to_file(out_path.join("builtins.rs"))?;

    #[cfg(feature = "modules")]
    builder
        .clone()
        .allowlist_file(file_regex!(src, "/Modules"))
        .generate()
        .map_err(io::Error::other)?
        .write_to_file(out_path.join("modules.rs"))?;

    builder
        .allowlist_recursively(true)
        .allowlist_file(file_regex!(src, ""))
        .allowlist_file(format!(r"{src}/.*?config\.h"))
        .generate()
        .map_err(io::Error::other)?
        .write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn generate_wrapper(source: &Path, out_dir: &Path) -> io::Result<PathBuf> {
    let wrapper = out_dir.join("wrapper.h");

    let content = glob::glob(&format!("{}/Src/**/*.mdh", source.display()))
        .map_err(io::Error::other)?
        .filter_map(Result::ok)
        .map(|path| path.strip_prefix(source).unwrap().to_path_buf())
        .fold(String::new(), |mut acc, p| {
            acc.push_str(&format!("#include \"{}\"\n", p.display()));
            acc
        });

    fs::write(&wrapper, content)?;

    Ok(wrapper)
}
