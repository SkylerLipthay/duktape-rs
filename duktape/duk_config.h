// This is a wrapper around the stock `duk_config.h` (renamed
// `duk_config_default.h` in this project) that allows for users of this crate
// to modify various configuration flags at compile-time (by means of Cargo
// features).

#if !defined(CUSTOM_DUK_CONFIG_H_INCLUDED)
#define CUSTOM_DUK_CONFIG_H_INCLUDED

#include "duk_config_default.h"

#ifdef RUST_DUK_PREVENT_TRACEBACKS
#undef DUK_USE_TRACEBACKS
#endif

#endif // CUSTOM_DUK_CONFIG_H_INCLUDED
