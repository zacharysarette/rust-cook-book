#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec: Rectangle<i32, f64> = Rectangle {
        length:4, height:10.5};
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
}
