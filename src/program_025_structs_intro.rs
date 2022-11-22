#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 234.50
    };
    bob.address = String::from("505 Main Street")
}
