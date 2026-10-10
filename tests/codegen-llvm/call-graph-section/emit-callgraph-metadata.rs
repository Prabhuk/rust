// Verifies that `-Zcall-graph-section` attaches `!callgraph` type identifier metadata to
// functions and `!callee_type` metadata to indirect call sites (but not to direct call sites),
// using the same type identifier for an indirect call site and its potential targets.
//
//@ add-minicore
//@ revisions: aarch64 x86_64
//@ [aarch64] compile-flags: --target aarch64-unknown-none
//@ [aarch64] needs-llvm-components: aarch64
//@ [x86_64] compile-flags: --target x86_64-unknown-none
//@ [x86_64] needs-llvm-components: x86
//@ compile-flags: -Cno-prepopulate-passes -Copt-level=0 -Zcall-graph-section
//@ min-llvm-version: 23

#![crate_type = "lib"]
#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

pub fn indirect(f: fn(i32) -> i32, arg: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}8indirect
    // CHECK-SAME:  {{.*}}!callgraph ![[INDIRECT_TYPE:[0-9]+]]
    // CHECK:       call {{(noundef )*}}i32 %f(i32 {{(noundef )*}}%arg){{.*}}!callee_type ![[CALLEE_TYPE:[0-9]+]]
    f(arg)
}

pub fn target(x: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}6target
    // CHECK-SAME:  {{.*}}!callgraph ![[TARGET_TYPE:[0-9]+]]
    x
}

pub fn direct(arg: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}6direct
    // CHECK-SAME:  {{.*}}!callgraph ![[TARGET_TYPE]]
    // CHECK:       call {{(noundef )*}}i32 @{{.*}}6target{{.*}}(i32 {{(noundef )*}}%arg)
    // CHECK-NOT:   !callee_type
    // CHECK:       ret i32
    target(arg)
}

// LLVM 23 requires call graph type identifiers to carry a `.generalized` suffix.
// CHECK-DAG: ![[INDIRECT_TYPE]] = !{!"_ZTSFu3i32PFS_S_ES_E{{(\.generalized)?}}"}
// CHECK-DAG: ![[TARGET_TYPE]] = !{!"_ZTSFu3i32S_E{{(\.generalized)?}}"}
// CHECK-DAG: ![[CALLEE_TYPE]] = !{![[TARGET_TYPE]]}
