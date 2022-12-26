#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use clap::Parser;

mod addremove;
mod simplediff;
mod output;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    color: bool,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Args::parse();
    let vec = simplediff::simplediff("yolo world", "hello world");
    if args.color {
        output::coloroutputdiff(vec);
    } else {
        output::simpleoutput(vec)
    }
}
