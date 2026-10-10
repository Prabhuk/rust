// Verifies that `-Zcall-graph-section` is rejected for targets that do not use the ELF object
// format, since LLVM only emits the `.llvm.callgraph` section for ELF.
//
//@ compile-flags: -Zcall-graph-section --target x86_64-pc-windows-msvc
//@ needs-llvm-components: x86

#![feature(no_core)]
#![no_core]
#![no_main]

//~? ERROR `-Zcall-graph-section` is only supported on targets that use the ELF object format
