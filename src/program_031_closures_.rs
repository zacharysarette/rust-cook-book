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
    // You can pass closures to functions
    // you can also create functions within functions
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}
