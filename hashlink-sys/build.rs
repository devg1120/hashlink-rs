use std::env;
use std::path::PathBuf;

use bindgen;
use cmake::Config;

use std::collections::HashSet;


#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn build_libhl_full()
{
    //let mut out_dir = PathBuf::from(env::var("out_dir").unwrap());
    let mut out_dir = PathBuf::from("./build");
    out_dir.push("_deploy");

    let mut build = cc::Build::new();

    // Global config
    build.cpp(true);
    build.static_flag(true);
    build.out_dir(out_dir);

    // C-Flags    
    build.flag("-fpermissive");
    build.flag("-std=c++11");

    // -I
    build.include("../vendor/hashlink/src");
    build.include("../vendor/hashlink/include/pcre");

    let std_src = [
          "wrapper.c"
    ];

    for source_file in &std_src {
        build.file(&source_file);
    }

    // Linker configuration 
    //build.static_crt(true);

    // -L
    build.flag("-L ./hashlink-sys/build/_deploy/");
    //build.flag("-L ./build/_deploy/");
    // -l
    build.flag("-lhlfull");
    
    // GO
    //build.compile("libhlfull.a");
    build.compile("hlfull");
}

fn main() {
    build_libhl_full();

    let ignored_macros = IgnoreMacros(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into(),
                "IPPORT_RESERVED".into(),
            ]
            .into_iter()
            .collect(),
        );

    //let mut out_dir = PathBuf::from(env::var("out_dir").unwrap());
    let mut out_dir = PathBuf::from("./build");
    out_dir.push("_deploy");

    //println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-search={}", "./hashlink-sys/build/_deploy");
    println!("cargo:rustc-link-lib=hlfull");
    //println!("cargo:rustc-link-lib=dylib=ws2_32");
    //println!("cargo:rustc-link-lib=dylib=user32");
    

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("--include-directory=../vendor/hashlink/src")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let mut bindings_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings_path.push("src");

    bindings
        .write_to_file(bindings_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");

}
