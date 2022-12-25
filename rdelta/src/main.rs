#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use colored::Colorize;
use itertools::Itertools;
use itertools::EitherOrBoth::{Left, Right, Both};
use unicode_segmentation::UnicodeSegmentation;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
}

// fn outputdiff(str: EitherOrBoth<&str, &str>) {
//
// }

fn simplediff(a: &str, b: &str) {
    let combined = a.graphemes(true).zip_longest(b.graphemes(true));
    for set in combined {
        match set {
            Both(l, r) if l == r => {
                print!("{}", l)
            }
            Both(l, r) => {
                print!("{}", l.red());
                print!("{}",r.green());
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
    let args = Args::parse();
    println!("Hello, world!");
    simplediff("yolo world", "hello world");
}
