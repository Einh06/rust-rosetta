// Implements http://rosettacode.org/wiki/Hello_world/Standard_error
#![cfg(not_tested)]
#![allow(unused_must_use)]

use std::io;

fn main() {
    let mut stderr = io::stderr();
    stderr.write(bytes!("Goodbye, World!\n"));
}
