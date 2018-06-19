extern crate cc;

#[cfg(feature = "prevent-tracebacks")]
const PREVENT_TRACEBACKS: bool = true;
#[cfg(not(feature = "prevent-tracebacks"))]
const PREVENT_TRACEBACKS: bool = false;

fn main() {
    let mut builder = cc::Build::new();

    builder.include("duktape")
        .flag("-std=c99")
        .file("duktape/duktape.c")
        .file("duktape/wrapper.c");

    if PREVENT_TRACEBACKS {
        builder.define("RUST_DUK_PREVENT_TRACEBACKS", None);
    }

    builder.compile("libduktape.a");
}
