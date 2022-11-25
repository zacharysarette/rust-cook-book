#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // BOX smart pointer stores data on the heap instead of the stack
    // Stack Last in First Out Format
    // Data on the stack must have a defined fixed size
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);
}
