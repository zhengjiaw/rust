extern crate cbindgen;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut file = File::open("cbindgen.toml").unwrap();
    let mut toml = String::new();
    file.read_to_string(&mut toml).unwrap();
    let config = toml::from_str(&toml).unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_config(config)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}
