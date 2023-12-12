#![allow(internal_features)]
#![feature(auto_traits, lang_items, no_core)]
#![no_std]
#![no_core]

// Making `Copy` an auto trait means it applies to `&mut T` for any `T`.
#[lang = "copy"]
pub auto trait Copy {}

// You can't compile unless a `Sized` trait is present.
#[lang = "sized"]
pub auto trait Sized {}

// Without the `extern` blocks, you get linker errors compiling the code.
#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {}
#[cfg(target_os = "macos")]
#[link(name = "System")]
extern "C" {}

// Implementers of this can be used as `self` parameters.
#[lang = "receiver"]
pub trait Receiver {}

// This lets us use `&mut T` as the `self` parameter in inherent impls.
impl<T: ?Sized> Receiver for &mut T {}

/// Rust doesn't know how to add without this trait, so we need it.
#[lang = "add_assign"]
pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

impl AddAssign for isize {
    fn add_assign(&mut self, rhs: Self) {
        // The fact that this doesn't recurse forever is pure magic.
        *self += rhs;
    }
}

#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
    let mut y = 23isize; // Create `y = 23`
    let x = &mut y; //      Create `x = &mut y`
    let z = x; //           `x` is copied into `z`
    *x += 2; //             `x` was copied, not moved, so we can still use it
    *z += 5; //             `z` is independent from `x` because it was copied, so we can use it
    y //                    Under proper circumstances, this is `23 + 2 + 5`, or `30`.
}

// We never call this, but we get linker and compiler errors without it.
fn main() {}
