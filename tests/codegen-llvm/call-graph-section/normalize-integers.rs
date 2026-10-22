// Verifies that `-Zsanitizer-cfi-normalize-integers` is honored by the call graph section type
// identifiers (like Clang honors `-fsanitize-cfi-icall-experimental-normalize-integers` for
// `-fexperimental-call-graph-section`), so that `extern "C"` function pointers and C functions
// compiled by Clang with integer normalization get matching type identifiers.
//
//@ add-minicore
//@ revisions: aarch64 x86_64
//@ [aarch64] compile-flags: --target aarch64-unknown-none
//@ [aarch64] needs-llvm-components: aarch64
//@ [x86_64] compile-flags: --target x86_64-unknown-none
//@ [x86_64] needs-llvm-components: x86
//@ compile-flags: -Cno-prepopulate-passes -Copt-level=0 -Zcall-graph-section -Zsanitizer-cfi-normalize-integers
//@ min-llvm-version: 24

#![crate_type = "lib"]
#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

pub extern "C" fn target(x: isize, b: bool) -> isize {
    // CHECK-LABEL: define{{.*}}6target
    // CHECK-SAME:  {{.*}}!callgraph ![[TARGET_TYPE:[0-9]+]]
    if b { x } else { 0 }
}

pub fn indirect(f: extern "C" fn(isize, bool) -> isize, arg: isize) -> isize {
    // CHECK-LABEL: define{{.*}}8indirect
    // CHECK:       call {{(noundef )*}}i64 %f(i64 {{(noundef )*}}%arg, i1 {{(noundef )*}}{{(zeroext )*}}true){{.*}}!callee_type ![[CALLEE_TYPE:[0-9]+]]
    f(arg, true)
}

// With integer normalization, `isize` is encoded as the fixed-width integer type of the same size
// and `bool` as `u8`, and the type identifier carries the `.normalized` suffix.
// CHECK-DAG: ![[TARGET_TYPE]] = !{!"_ZTSFu3i64S_u2u8E.normalized"}
// CHECK-DAG: ![[CALLEE_TYPE]] = !{![[TARGET_TYPE]]}
