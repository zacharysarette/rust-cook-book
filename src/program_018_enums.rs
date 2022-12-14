#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum Days {
        Monday, 
        Tuesday, 
        Wednesday, 
        Thursday, 
        Friday, 
        Saturday, 
        Sunday 
    }

    impl Days {
       fn is_weekend(&self) -> bool {
        match self {
            Days::Saturday | Days::Sunday => true,
            _=> false
        }
       } 
    }

    let today:Days = Days::Sunday;

    match today {
        Days::Monday => println!("Everyone hates Monday"),
        Days::Tuesday => println!("Donut Day"),
        Days::Wednesday => println!("Hump Day"),
        Days::Thursday => println!("Pay Day"),
        Days::Friday => println!("Fire Friday!"),
        Days::Saturday => println!("Cartoon and cereal day"),
        Days::Sunday => println!("Rest Day"),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}
