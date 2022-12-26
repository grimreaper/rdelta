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



enum AddRemove {
    Add(String),
    Same(String),
    Remove(String),
    Replace(String, String),
}

fn coloroutputdiff<'a>(out: impl Iterator<Item=AddRemove>) -> () {
    for item in out {
        match item {
            AddRemove::Add(s) => {
                print!("{}", s.green());
            }
            AddRemove::Remove(s) => {
                print!("{}", s.red());
            }
            AddRemove::Same(s) => {
                print!("{}", s);
            }
            AddRemove::Replace(l, r) => {
                print!("{}", l.red());
                print!("{}", r.green());
            }
        }
    }
    print!("\n")
}

fn simplediff<'l>(a: &'l str, b: &'l str) -> impl Iterator<Item=AddRemove> {
    let combined = a.graphemes(true).zip_longest(b.graphemes(true));
    let mut output: Vec<AddRemove> = Vec::new();
    for set in combined {
        match set {
            Both(l, r) if l == r => {
                output.push(AddRemove::Same(l.to_string()))
            }
            Both(l, r) => {
                output.push(AddRemove::Replace(l.to_string(), r.to_string()));
            }
            Left(l) => {
                output.push(AddRemove::Remove(l.to_string()));
            }
            Right(r) => {
                output.push(AddRemove::Remove(r.to_string()));
            }
        }
    }
    output.into_iter()
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    let vec= simplediff("yolo world", "hello world");
    coloroutputdiff(vec);
}
