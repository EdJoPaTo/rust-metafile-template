use std::env;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;

use clap_generate::{generate_to, generators};

include!("src/cli.rs");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = Path::new("completions/");
    drop(remove_dir_all(out_dir));
    create_dir_all(out_dir).unwrap();

    let mut app = build();
    let bin_name = env!("CARGO_PKG_NAME");

    generate_to(generators::Bash, &mut app, bin_name, out_dir).unwrap();
    generate_to(generators::Elvish, &mut app, bin_name, out_dir).unwrap();
    generate_to(generators::Fish, &mut app, bin_name, out_dir).unwrap();
    generate_to(generators::PowerShell, &mut app, bin_name, out_dir).unwrap();
    generate_to(generators::Zsh, &mut app, bin_name, out_dir).unwrap();
}
