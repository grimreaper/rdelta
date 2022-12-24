use colored::Colorize;
use itertools::{EitherOrBoth::*, Itertools};
use unicode_segmentation::UnicodeSegmentation;

fn simplediff(a: &str, b: &str) {
    let combined = a.graphemes(true).zip(b.graphemes(true)).zip_longest();
    for set in combined {
        match combined {
            Both(l, r) => {
                if (l == r) {
                    print!(l)
                } else {
                    print!(l.red());
                    print!(r.green());
                }
            }
            Left(l) => {
                print!(l.red());
            }
            Right(r) => {
                print!(r.green);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
