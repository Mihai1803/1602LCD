//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("./memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={{layout}}");

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    println!("cargo:rustc-link-arg=--nmagic");

    // The `link.x` linker script provided by `cortex_m_rt` (minimal runtime for
    // Cortex-M microcontrollers used by embassy) will include our `memory.x` memory layout.
    println!("cargo:rustc-link-arg=-Tlink.x");

    // The `link-rp.x` linker script provided by `embassy_rp` that defines the
    // BOOT2 section.
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");

    // The `defmt.x` linker script provided by `defmt`.
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}