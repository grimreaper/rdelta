#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};

mod addremove;
mod output;
mod simplediff;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    color: bool,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = wild::args_os();
    let args = argfile::expand_args_from(args, argfile::parse_fromfile, argfile::PREFIX).unwrap();
    let matches = Args::parse_from(args);
    let vec = simplediff::simplediff("yolo world", "hello world");
    let output: String = {
        if matches.color {
            output::coloroutputdiff(vec)
        } else {
            output::simpleoutput(vec)
        }
    };
    println!("{}", output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
