#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // let var_name = |parameters| -> return_type
    // {BODY}
    // closures
    // closures can access variables outside of it's body unlike functions
    let mut samp1 = 5;
    let print_var = || println!("Sample One : {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("Sample One : {}", samp1);
    samp1 = 10;
    println!("Sample One : {}", samp1);
}
