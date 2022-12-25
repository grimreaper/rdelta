#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use std::borrow::Borrow;
use std::ops::Deref;
use std::slice::Iter;
use colored::Colorize;
use itertools::Itertools;
use itertools::EitherOrBoth::{Left, Right, Both};
use unicode_segmentation::UnicodeSegmentation;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

// struct {}

fn outputdiff<'a>(out: impl Iterator<Item=String>) -> () {
    for s in out {
        print!("{}", s)
    }
    print!("\n")
}

fn simplediff<'l>(a: &'l str, b: &'l str) -> impl Iterator<Item=String> {
    let combined = a.graphemes(true).zip_longest(b.graphemes(true));
    let mut output: Vec<String> = Vec::new();
    for set in combined {
        match set {
            Both(l, r) if l == r => {
                output.push(l.to_string())
            }
            Both(l, r) => {
                output.push(l.red().to_string());
                output.push(r.green().to_string());
            }
            Left(l) => {
                output.push(l.red().to_string());
            }
            Right(r) => {
                output.push(r.green().to_string());
            }
        }
    }
    output.into_iter()
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    let vec= simplediff("yolo world", "hello world");
    outputdiff(vec);
}
