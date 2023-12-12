#![allow(internal_features)]
#![feature(auto_traits, lang_items, no_core, rustc_attrs)]
#![no_core]
#![no_std]

#[lang = "copy"]
pub auto trait Copy {}

#[lang = "sized"]
pub auto trait Sized {}

#[cfg(target_os = "linux")]
#[link(name = "c")]
extern "C" {}

#[cfg(target_os = "macos")]
#[link(name = "System")]
extern "C" {}

#[lang = "receiver"]
pub trait Receiver {}

impl<T: ?Sized> Receiver for &mut T {}

#[lang = "add_assign"]
pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

impl AddAssign for isize {
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs;
    }
}

#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
    let mut y = 23isize;
    let x = &mut y;
    let z = x;
    *x += 2;
    *z += 5;
    y
}

fn main() {}
