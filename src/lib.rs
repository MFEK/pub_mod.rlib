#![feature(proc_macro_quote)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

use std::fs;
use std::path::PathBuf;

// https://stackoverflow.com/a/59330922/1901658
fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}

#[proc_macro]
pub fn pub_mod(input_ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_ts as LitStr);
    let dir: PathBuf = input.value().into();

    let mut modules = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        match entry.path().file_name() {
            Some(file) => {
                let filename = file.to_str().unwrap();
                if filename == "lib.rs" || filename == "mod.rs" { continue }
                modules.push(remove_suffix(filename, ".rs").to_string())
            },
            None => {},
        }
    }

    let mut pub_mod = String::new();

    for module in modules.iter() {
        pub_mod.push_str(&format!("pub mod {};\n", &module));
    }

    pub_mod.parse().unwrap()
}
