// `-Z call-graph-section` makes LLVM emit a `.llvm.callgraph` section (the
// counterpart of Clang's `-fexperimental-call-graph-section`) that describes
// the direct and indirect calls of each function. This test compiles a crate
// with the flag and checks that the output object file contains the section.

//@ needs-target-std
//@ only-elf
// Reason: this feature only works when the output object format is ELF.
//@ min-llvm-version: 24
// Reason: the type identifiers emitted for the call graph section are only accepted by LLVM 24.
//@ ignore-backends: gcc

use run_make_support::{llvm_readobj, rustc};

fn main() {
    rustc().opt_level("3").arg("-Zcall-graph-section").emit("obj").input("foo.rs").run();
    llvm_readobj()
        .arg("--section-headers")
        .input("foo.o")
        .run()
        .assert_stdout_contains(".llvm.callgraph");
}
