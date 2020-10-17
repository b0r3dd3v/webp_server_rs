use std::path::Path;
fn main() {
    
     let clang = Path::new("/usr/bin/clang");
     let fille = Path::new("./webpwrapper/webwrapper.c");
     let inclu3de = Path::new("./deps/include");
     let depthlib = Path::new("./deps/lib/libwebpwrapper.a");
     cc::Build::new()
        .compiler(clang)
        .no_default_flags(true)
        .file(fille)
        .pic(true)
        .static_flag(true)
        .opt_level(3)
        .include(inclu3de)
        .compile(depthlib);
    println!("cargo:rustc-link-search=./deps/lib");
}
