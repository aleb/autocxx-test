use autocxx::prelude::*;

include_cpp! {
    #include "aaa.hh"
    safety!(unsafe)
    generate!("Sandwich")
}

fn main() {
    println!("Hello, world!");
}
