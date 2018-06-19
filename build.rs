extern crate cc;

fn main() {
    let mut builder = cc::Build::new();

    builder.include("duktape")
        .flag("-std=c99")
        .file("duktape/duktape.c")
        .file("duktape/wrapper.c");

    if cfg!(feature = "use-tracebacks") {
        builder.define("RUST_DUK_USE_TRACEBACKS", None);
    }

    if cfg!(feature = "use-augment-error-create") {
        builder.define("RUST_DUK_USE_AUGMENT_ERROR_CREATE", None);
    }

    builder.compile("libduktape.a");
}
