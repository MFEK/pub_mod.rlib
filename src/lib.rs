use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

use std::fs;
use std::path::PathBuf;

#[proc_macro]
pub fn pub_mod(input_ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_ts as LitStr);
    let dir: PathBuf = input.value().into();

    let mut modules = vec![];

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if let Some(file) = entry.path().file_name() {
            let filename = file.to_str().unwrap();
            if filename == "lib.rs" || filename == "mod.rs" { continue }
            modules.push(filename.strip_suffix(".rs").unwrap_or(filename).to_string())
        }
    }

    let mut pub_mod = String::new();

    for module in modules.iter() {
        pub_mod.push_str(&format!("pub mod {};\n", &module));
    }

    pub_mod.parse().unwrap()
}
