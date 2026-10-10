# `call-graph-section`

------------------------

The rustc flag `-Z call-graph-section` makes LLVM emit a `.llvm.callgraph`
section into the output object files. It is the Rust counterpart of Clang's
`-fexperimental-call-graph-section` flag and shares the same LLVM backend
implementation, so Rust and C/C++ objects can be linked together and analyzed
uniformly.

The section contains one record per function describing:

- whether the function is a potential indirect call target and, if so, a
  numeric type identifier for its signature,
- the entry points of all functions it calls directly, and
- the type identifiers of all indirect call sites it contains.

Tools such as `llvm-readelf --call-graph-info` consume this section (typically
together with the `.stack_sizes` section emitted by `-Z emit-stack-sizes`) to
reconstruct the call graph of a binary, including the possible targets of
indirect calls, for example to compute worst-case stack usage.

> **NOTE**: The call graph section is only defined for the ELF object format.
> Using this flag with targets that use other object formats (e.g. macOS and
> Windows) is an error. LLVM 23 or newer is required.

## Type identifiers

The type identifiers of indirect call sites and of their potential targets are
computed from the function signature, using the same Itanium C++ ABI based
encoding as the Rust LLVM CFI support (see `-Zsanitizer=cfi`). An indirect call
site is associated with every function that carries the same type identifier.
For this purpose, methods use the signature with the self type erased, so that
calls through trait objects (`dyn Trait`) match the implementations of the
trait method.

Like Clang, the `-Zsanitizer-cfi-normalize-integers` option is honored when
computing the type identifiers, and is accepted together with
`-Z call-graph-section` even when no sanitizer is enabled. It allows the type
identifiers of `extern "C"` function pointers and of C functions compiled by
Clang with `-fsanitize-cfi-icall-experimental-normalize-integers` to match.

## Example

Consider this crate:

```rust
#![crate_type = "lib"]

pub fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[inline(never)]
pub fn double(x: i32) -> i32 {
    x * 2
}

pub fn run() -> i32 {
    apply(double, 21)
}
```

Using the `-Z call-graph-section` flag produces an extra `.llvm.callgraph`
section (linked to the function's text section) for every function in the
output *object file*:

```console
$ rustc -C opt-level=3 --emit=obj -Z call-graph-section foo.rs

$ llvm-readelf --sections foo.o
...
  [ 3] .text._RNvCs3Dnng3aZVPH_3foo3run PROGBITS 0000000000000000 000040 00000b 00  AX  0   0 16
  [ 5] .llvm.callgraph   LLVM_CALL_GRAPH 0000000000000000 00004b 00001b 00   L  3   0  1
  [ 7] .text._RNvCs3Dnng3aZVPH_3foo5apply PROGBITS 0000000000000000 000068 000007 00  AX  0   0  8
  [ 8] .llvm.callgraph   LLVM_CALL_GRAPH 0000000000000000 00006f 00001b 00   L  7   0  1
  [10] .text._RNvCs3Dnng3aZVPH_3foo6double PROGBITS 0000000000000000 00008c 000004 00  AX  0   0  4
  [11] .llvm.callgraph   LLVM_CALL_GRAPH 0000000000000000 000090 000012 00   L 10   0  1
...
```

The call graph information can be inspected with
`llvm-readelf --call-graph-info --elf-output-style=JSON foo.o`. In the output,
`run` lists `double` as a direct callee (`apply` got inlined into it), the
record for `apply` lists the type identifier of its indirect call site
(`fn(i32) -> i32`), and the record for `double` carries the same type
identifier, which marks it as a potential target of that indirect call.

In Cargo projects the standard library can be recompiled with the flag using
the nightly [build-std](../../cargo/reference/unstable.html#build-std) feature
so that the call graph also covers the standard library.
