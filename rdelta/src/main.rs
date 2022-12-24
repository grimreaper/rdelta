#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use colored::Colorize;
use itertools::{EitherOrBoth::*, Itertools};
use unicode_segmentation::UnicodeSegmentation;

fn simplediff(a: &str, b: &str) {
    let combined = a.graphemes(true).zip_longest(b.graphemes(true));
    for set in combined {
        match set {
            Both(l, r) => {
                if l == r {
                    print!("{}", l)
                } else {
                    print!("{}", l.red());
                    print!("{}",r.green());
                }
            }
            Left(l) => {
                print!("{}",l.red());
            }
            Right(r) => {
                print!("{}", r.green());
            }
        }
    }
    print!("\n")
}

fn main() {
    println!("Hello, world!");
    simplediff("yolo world", "hello world");
}
