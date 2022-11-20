#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let num_1: f32 = 1.1111111111111111;
    println!("f32: {}", num_1 + 0.11111111111111111);
    let num_2: f64 = 1.1111111111111111;
    println!("f64: {}", num_2 + 0.11111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4: {}", num_3 + num_4);
    println!("5 - 4: {}", num_3 - num_4);
    println!("5 * 4: {}", num_3 * num_4);
    println!("5 / 4: {}", num_3 / num_4);
    println!("5 % 4: {}", num_3 % num_4);
    let mut num_3_1:u32 = 1; 
    num_3_1 += 1;
    println!("mutable 1 += 1 => {}", num_3_1);
}
