// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

// Make sure we actually link in userlib, despite not using any of it explicitly
// - we need it for our _start routine.

use heapless::String;
extern crate userlib;

#[export_name = "main"]
fn main() -> ! {
    loop {
        example12();
    }
}

pub trait Frobber {
    fn frob(&self, add: u32) -> String<50>;
}

pub struct Foo {
    var1: u32
}

impl Frobber for Foo {
    fn frob(&self, add: u32) -> String<50> {
        if self.var1 + add == 10 {
            return String::from("[Foo.10]");
        } else {
            return String::from("[Foo.else]");
        }
    }
}

pub struct Bar {
    var2: i32
}

impl Frobber for Bar {
    fn frob(&self, add: u32) -> String<50> {
        if self.var2 + add as i32 == -10 {
            return String::from("[Bar.-10]");
        } else {
            return String::from("[Bar.else]");
        }
    }
}

/// `frob_it_static` is effectively a _family_ of functions parameterized
/// by type `T`. To a first approximation, the Rust compiler generates a
/// separate function body for unique `T`. Since Rust does not support
/// inheritance/subtype-style polymorphism, the `frobber` argument of
/// `frob_it_static` instantiated for a concrete type, e.g. `Foo`, will
/// accept values _only_ of exactly type `Foo`. This feels very much like
/// Haskell's [_parametric polymorphism_](https://wiki.haskell.org/Polymorphism).
/// This is similar to [templates](https://en.cppreference.com/w/cpp/language/templates)
/// with the addition of [constraints and concepts](https://en.cppreference.com/w/cpp/language/constraints).
/// In `frob_it_static`, `T: Frobber` is a trait constraint specifying that
/// `T` must implement this constraint.
#[inline(never)]
fn frob_it_static<T: Frobber>(frobber: T, add: u32) -> String<50> {
    frobber.frob(add)
}

/// The `dyn` prefix is optional here (the compiler will warn if it is absent).
/// Regardless, `dyn` is implied and this is inherently dynamically dispatched.
/// The type of the argument must be a reference specified with `&`. You cannot
/// a `Frobber` by value: `Frobber` is a trait so there is no such thing as a
/// _value of type `Frobber`_. Notice how `frob_it_dynamic` has no type arguments.
/// Therefore, this defines a single _concrete_ function as opposed to a family
/// of functions.
#[inline(never)]
fn frob_it_dynamic(frobber: &dyn Frobber, add: u32) -> String<50> {
    frobber.frob(add)
}

#[no_mangle]
#[inline(never)]
fn example12() -> bool {
    // Static binding
    if frob_it_static(Foo{var1: 9}, 1) != "[Foo.10]" {
        return false;
    }
    if frob_it_static(Foo{var1: 11}, 2) != "[Foo.else]" {
        return false;
    }
    if frob_it_static(Bar{var2: -11}, 1) != "[Bar.-10]" {
        return false;
    }
    if frob_it_static(Bar{var2: 10}, 3) != "[Bar.else]" {
        return false;
    }
    // Dinamic binding
    if frob_it_dynamic(&Foo{var1: 9}, 1) != "[Foo.10]" {
        return false;
    }
    if frob_it_dynamic(&Foo{var1: 11}, 2) != "[Foo.else]" {
        return false;
    }
    if frob_it_dynamic(&Bar{var2: -11}, 1) != "[Bar.-10]" {
        return false;
    }
    if frob_it_dynamic(&Bar{var2: 10}, 3) != "[Bar.else]" {
        return false;
    }
    return true;
}