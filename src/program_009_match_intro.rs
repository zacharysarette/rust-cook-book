#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday!"),
        21 | 50 => println!("Important Birthday!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("Not an Important Birthday!"),
    };
}
