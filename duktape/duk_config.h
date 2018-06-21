// This is a wrapper around the stock `duk_config.h` (renamed
// `duk_config_default.h` in this project) that allows for users of this crate
// to modify various configuration flags at compile-time (by means of Cargo
// features).

#if !defined(CUSTOM_DUK_CONFIG_H_INCLUDED)
#define CUSTOM_DUK_CONFIG_H_INCLUDED

#include "duk_config_default.h"

#ifndef RUST_DUK_USE_TRACEBACKS
#undef DUK_USE_TRACEBACKS
#endif

#ifndef RUST_DUK_USE_AUGMENT_ERROR_CREATE
#undef DUK_USE_AUGMENT_ERROR_CREATE
#endif

#ifdef RUST_DUK_USE_EXEC_TIMEOUT_CHECK
#define DUK_USE_INTERRUPT_COUNTER
#define DUK_USE_EXEC_TIMEOUT_CHECK(udata) (duk_get_exec_timeout_function())((udata))
typedef duk_bool_t (*duk_exec_timeout_function)(void *udata);
void duk_set_exec_timeout_function(duk_exec_timeout_function func);
duk_exec_timeout_function duk_get_exec_timeout_function();
#endif

#endif // CUSTOM_DUK_CONFIG_H_INCLUDED
