#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let is_true: bool = true;
    let my_grade = 'A';
    print!("is true: {}, my grade: {}", is_true, my_grade);
}
