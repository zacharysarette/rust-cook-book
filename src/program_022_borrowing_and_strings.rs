#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_str(name: &mut String) {
    name.push_str(" is beautiful");
    println!("A string {}", name);
}

fn main() {
    let str1 = String::from("Beetle Juice");
    let str2 = str1.clone(); // if you don't use clone, str1 will be lost in memory
    println!("What is string one? Answer: {}", str1);

    // print_str(str1);

    // let str3: String = print_return_str(str1);
    // println!("string 3 is {}",str3);

    let mut str4 = String::from("Buzz Lightbeer");
    change_str(&mut str4);
    println!("string 4 is {}",str4);


}
