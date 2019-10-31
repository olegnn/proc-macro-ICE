extern crate proc_macro_bug_impl;
use proc_macro_bug_impl::some_macro;

#[some_macro(0)]
struct Abc {}

#[some_macro(0)]
struct Cde {}

fn main() {}
