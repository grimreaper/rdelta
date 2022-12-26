use colored::Colorize;
use crate::addremove::AddRemove;

pub fn coloroutputdiff<'a>(out: impl Iterator<Item = AddRemove>) -> () {
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

pub fn simpleoutput<'a>(out: impl Iterator<Item = AddRemove>) -> () {
    for item in out {
        match item {
            AddRemove::Add(s) => {
                print!("+{}", s);
            }
            AddRemove::Remove(s) => {
                print!("-{}",s );
            }
            AddRemove::Same(s) => {
                print!("{}", s);
            }
            AddRemove::Replace(rem, add) => {
                print!("-{}+{}", rem, add);
            }
        }
    }
    print!("\n")
}
