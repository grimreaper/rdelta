#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![deny(missing_abi)]
#![warn(missing_copy_implementations)]

use clap::Parser;

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
    let args = Args::parse();
    let vec = simplediff::simplediff("yolo world", "hello world");
    let output: String = {
        if args.color {
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
