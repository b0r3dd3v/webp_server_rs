use std::path::Path;
fn main() {
     cc::Build::new()
        .compiler("/usr/bin/clang")
        .no_default_flags(true)
        .file("webpwrapper.c")
        .pic(true)
        .static_flag(true)
        .opt_level(3)
        .include("/usr/include/webp")
        .compile("/tmp/lib/libwebpwrapper.a");
    println!("cargo:rustc-link-search=/tmp/lib");
}
