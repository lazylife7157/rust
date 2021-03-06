// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// revisions: lxl nll
//[lxl]compile-flags: -Z borrowck=mir -Z two-phase-borrows
//[nll]compile-flags: -Z borrowck=mir -Z two-phase-borrows -Z nll

// This is the first counter-example from Niko's blog post
// smallcultfollowing.com/babysteps/blog/2017/03/01/nested-method-calls-via-two-phase-borrowing/
// of a danger for code to crash if we just turned off the check for whether
// a mutable-borrow aliases another borrow.

fn main() {
    let mut v: Vec<String> = vec![format!("Hello, ")];
    v[0].push_str({

        v.push(format!("foo"));
        //[lxl]~^ ERROR cannot borrow `v` as mutable more than once at a time [E0499]
        //[nll]~^^   ERROR cannot borrow `v` as mutable more than once at a time [E0499]

        "World!"
    });
}
