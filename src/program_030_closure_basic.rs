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
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));

}
