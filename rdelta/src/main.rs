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
use itertools::EitherOrBoth::{Both, Left, Right};
use unicode_segmentation::UnicodeSegmentation;

use clap::Parser;

mod addremove;
mod simplediff;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    let vec= simplediff::simplediff("yolo world", "hello world");
    simplediff::coloroutputdiff(vec);
}
