#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let my_tuple: (u8, String, f64) = (36, "Zach".to_string(),
50_000.00);

println!("Name : {}", my_tuple.1);
let(v1, v2, v3) = my_tuple;
println!("Age : {}", v1);

}
