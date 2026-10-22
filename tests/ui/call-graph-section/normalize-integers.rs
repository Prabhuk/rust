// Verifies that `-Zsanitizer-cfi-normalize-integers` is accepted together with
// `-Zcall-graph-section` without enabling CFI or KCFI, like Clang accepts
// `-fsanitize-cfi-icall-experimental-normalize-integers` with `-fexperimental-call-graph-section`.
//
//@ check-pass
//@ compile-flags: -Zcall-graph-section -Zsanitizer-cfi-normalize-integers --target x86_64-unknown-linux-gnu
//@ needs-llvm-components: x86
//@ min-llvm-version: 24

#![feature(no_core)]
#![no_core]
#![no_main]
