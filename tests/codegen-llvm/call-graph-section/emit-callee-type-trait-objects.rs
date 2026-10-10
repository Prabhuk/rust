// Verifies that, with `-Zcall-graph-section`, calls through trait objects and the implementations
// of the trait methods they can reach share the same type identifier (i.e., the self type is
// erased in the type identifier of methods, like in the primary LLVM CFI type identifier).
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

pub trait Trait1 {
    fn foo(&self);
}

pub struct Type1;

impl Trait1 for Type1 {
    // CHECK-LABEL: define{{.*}}3foo
    // CHECK-SAME:  {{.*}}!callgraph ![[TYPE1:[0-9]+]]
    fn foo(&self) {}
}

pub trait Trait2<T> {
    fn bar(&self, _: T) -> T;
}

pub struct Type2;

impl Trait2<i32> for Type2 {
    // CHECK-LABEL: define{{.*}}3bar
    // CHECK-SAME:  {{.*}}!callgraph ![[TYPE2:[0-9]+]]
    fn bar(&self, x: i32) -> i32 {
        x
    }
}

pub fn call_foo(a: &dyn Trait1) {
    // CHECK-LABEL: define{{.*}}8call_foo
    // CHECK:       call void {{%f|%[0-9]+}}(ptr {{(noundef )*}}{{(align [0-9]+ )*}}{{%a.0|%[0-9]+}}){{.*}}!callee_type ![[CALLEE_TYPE1:[0-9]+]]
    a.foo();
}

pub fn call_bar(a: &dyn Trait2<i32>, x: i32) -> i32 {
    // CHECK-LABEL: define{{.*}}8call_bar
    // CHECK:       call {{(noundef )*}}i32 {{%f|%[0-9]+}}(ptr {{(noundef )*}}{{(align [0-9]+ )*}}{{%a.0|%[0-9]+}}, i32 {{(noundef )*}}%x){{.*}}!callee_type ![[CALLEE_TYPE2:[0-9]+]]
    a.bar(x)
}

pub fn make_dyn() -> (&'static dyn Trait1, &'static dyn Trait2<i32>) {
    (&Type1, &Type2)
}

// CHECK-DAG: ![[TYPE1]] = !{!"_ZTSFvu3refIu3dynIu{{[0-9]+}}NtC{{[[:print:]]+}}_{{[[:print:]]+}}6Trait1u6regionEEE{{(\.generalized)?}}"}
// CHECK-DAG: ![[CALLEE_TYPE1]] = !{![[TYPE1]]}
// CHECK-DAG: ![[TYPE2]] = !{!"_ZTSFu3i32u3refIu3dynIu{{[0-9]+}}NtC{{[[:print:]]+}}_{{[[:print:]]+}}6Trait2IS_Eu6regionEES_E{{(\.generalized)?}}"}
// CHECK-DAG: ![[CALLEE_TYPE2]] = !{![[TYPE2]]}
