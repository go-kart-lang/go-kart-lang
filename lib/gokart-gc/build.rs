use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("gokart_gc.h")
        // Generate constants that match c_int as i32.
        // See: https://users.rust-lang.org/t/interfacing-c-code-with-bindgen-define-and-types/67595
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
