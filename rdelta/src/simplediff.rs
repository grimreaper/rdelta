use itertools::EitherOrBoth::{Both, Left, Right};
use colored::Colorize;
use unicode_segmentation::UnicodeSegmentation;
use itertools::Itertools;
use crate::addremove::AddRemove;

pub fn coloroutputdiff<'a>(out: impl Iterator<Item=AddRemove>) -> () {
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

pub fn simplediff<'l>(a: &'l str, b: &'l str) -> impl Iterator<Item=AddRemove> {
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
