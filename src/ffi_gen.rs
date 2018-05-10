extern crate bindgen;

use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    write_wrapper_header().expect("generating wrapper.h failed");
    write_wrapper_c_file().expect("generating wrapper.c failed");
    write_bindgen();
}

fn write_wrapper_header() -> io::Result<()> {
    let mut header_file = File::create("duktape/wrapper.h")?;

    writeln!(header_file, "#include \"duktape.h\"")?;

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(header_file, "")?;
        writeln!(header_file, "#pragma push_macro({:?})", n)?;
        writeln!(header_file, "#undef {}", n)?;
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(header_file, "{} {}({});", rt, n, params)?;
        writeln!(header_file, "#pragma pop_macro({:?})", n)?;
    }

    Ok(())
}

fn write_wrapper_c_file() -> io::Result<()> {
    let mut c_file = File::create("duktape/wrapper.c")?;

    writeln!(c_file, "#include \"wrapper.h\"")?;

    for &(rt, n, ps) in MACRO_FUNCTIONS {
        writeln!(c_file, "")?;
        writeln!(c_file, "#pragma push_macro({:?})", n)?;
        writeln!(c_file, "#undef {}", n)?;
        let params = join(ps.iter().map(|&(pt, pn)| format!("{} {}", pt, pn)), ", ");
        writeln!(c_file, "{} {}({}) {{", rt, n, params)?;
        writeln!(c_file, "#pragma pop_macro({:?})", n)?;
        let args = join(ps.iter().map(|&(_, pn)| pn), ", ");
        let maybe_return = if rt == "void" { "" } else { "return " };
        writeln!(c_file, "  {}{}({});", maybe_return, n, args)?;
        writeln!(c_file, "}}")?;
    }

    Ok(())
}

fn write_bindgen() {
    bindgen::Builder::default()
        .header("duktape/wrapper.h")
        .clang_arg("-Iduktape")
        .clang_arg("-std=c99")
        .rust_target(bindgen::RustTarget::Stable_1_25)
        .whitelist_type("^duk_.*")
        .whitelist_function("^duk_.*")
        .whitelist_var("^DUK_.*")
        .generate()
        .expect("failed to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("failed to write bindings");
}

fn join<S, I>(iter: I, sep: &str) -> String where S: Borrow<str>, I: Iterator<Item = S> {
    iter.collect::<Vec<_>>().join(sep)
}

type MacroFunctionDescRet = &'static str;
type MacroFunctionDescName = &'static str;
type MacroFunctionDescArgs = &'static [(&'static str, &'static str)];
type MacroFunctionDesc = (MacroFunctionDescRet, MacroFunctionDescName, MacroFunctionDescArgs);

const MACRO_FUNCTIONS: &'static [MacroFunctionDesc] = &[
    ("duk_context *", "duk_create_heap_default",
     &[]),

    ("void", "duk_xmove_top",
     &[("duk_context *", "to_ctx"), ("duk_context *", "from_ctx"), ("duk_idx_t", "count")]),

    ("void", "duk_xcopy_top",
     &[("duk_context *", "to_ctx"), ("duk_context *", "from_ctx"), ("duk_idx_t", "count")]),

    ("const char *", "duk_push_string_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("duk_idx_t", "duk_push_thread",
     &[("duk_context *", "ctx")]),

    ("duk_idx_t", "duk_push_thread_new_globalenv",
     &[("duk_context *", "ctx")]),

    ("duk_idx_t", "duk_push_error_object",
     &[("duk_context *", "ctx"), ("duk_errcode_t", "err_code"), ("const char *", "fmt")]),

    ("void *", "duk_push_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size"), ("duk_bool_t", "dynamic")]),

    ("void *", "duk_push_fixed_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size")]),

    ("void *", "duk_push_dynamic_buffer",
     &[("duk_context *", "ctx"), ("duk_size_t", "size")]),

    ("void", "duk_push_external_buffer",
     &[("duk_context *", "ctx")]),

    ("duk_bool_t", "duk_is_callable",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_primitive",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_object_coercible",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_eval_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_range_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_reference_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_syntax_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_type_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("duk_bool_t", "duk_is_uri_error",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void", "duk_require_type_mask",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_uint_t", "mask")]),

    ("void", "duk_require_callable",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void", "duk_require_object_coercible",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void *", "duk_to_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),

    ("void *", "duk_to_fixed_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),

    ("void *", "duk_to_dynamic_buffer",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index"), ("duk_size_t *", "out_size")]),

    ("const char *", "duk_safe_to_string",
     &[("duk_context *", "ctx"), ("duk_idx_t", "index")]),

    ("void", "duk_eval",
     &[("duk_context *", "ctx")]),

    ("void", "duk_eval_noresult",
     &[("duk_context *", "ctx")]),

    ("duk_int_t", "duk_peval",
     &[("duk_context *", "ctx")]),

    ("duk_int_t", "duk_peval_noresult",
     &[("duk_context *", "ctx")]),

    ("void", "duk_compile",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags")]),

    ("duk_int_t", "duk_pcompile",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags")]),

    ("void", "duk_eval_string",
     &[("duk_context *", "ctx"), ("const char *", "src")]),

    ("void", "duk_eval_string_noresult",
     &[("duk_context *", "ctx"), ("const char *", "src")]),

    ("duk_int_t", "duk_peval_string",
     &[("duk_context *", "ctx"), ("const char *", "src")]),

    ("duk_int_t", "duk_peval_string_noresult",
     &[("duk_context *", "ctx"), ("const char *", "src")]),

    ("void", "duk_compile_string",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),

    ("void", "duk_compile_string_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),

    ("duk_int_t", "duk_pcompile_string",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),

    ("duk_int_t", "duk_pcompile_string_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "src")]),

    ("void", "duk_eval_lstring",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),

    ("void", "duk_eval_lstring_noresult",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),

    ("duk_int_t", "duk_peval_lstring",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),

    ("duk_int_t", "duk_peval_lstring_noresult",
     &[("duk_context *", "ctx"), ("const char *", "buf"), ("duk_size_t", "len")]),

    ("void", "duk_compile_lstring",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"),
     ("duk_size_t", "len")]),

    ("void", "duk_compile_lstring_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"),
     ("duk_size_t", "len")]),

    ("duk_int_t", "duk_pcompile_lstring",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"),
     ("duk_size_t", "len")]),

    ("duk_int_t", "duk_pcompile_lstring_filename",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "buf"),
     ("duk_size_t", "len")]),

    ("void", "duk_eval_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("void", "duk_eval_file_noresult",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("duk_int_t", "duk_peval_file",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("duk_int_t", "duk_peval_file_noresult",
     &[("duk_context *", "ctx"), ("const char *", "path")]),

    ("void", "duk_compile_file",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "path")]),

    ("duk_int_t", "duk_pcompile_file",
     &[("duk_context *", "ctx"), ("duk_uint_t", "flags"), ("const char *", "path")]),

    ("void", "duk_dump_context_stdout",
     &[("duk_context *", "ctx")]),

    ("void", "duk_dump_context_stderr",
     &[("duk_context *", "ctx")]),
];
