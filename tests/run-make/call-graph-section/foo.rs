#![crate_type = "lib"]

#[inline(never)]
pub fn foo(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

#[inline(never)]
pub fn bar(x: i32) -> i32 {
    x + 1
}
