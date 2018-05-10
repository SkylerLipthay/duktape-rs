extern crate cc;

fn main() {
    cc::Build::new()
        .include("duktape")
        .flag("-std=c99")
        .file("duktape/duktape.c")
        .file("duktape/wrapper.c")
        .compile("libduktape.a");
}
