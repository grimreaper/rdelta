use itertools::EitherOrBoth::{Both, Left, Right};
use colored::Colorize;
use unicode_segmentation::UnicodeSegmentation;
use itertools::Itertools;

pub enum AddRemove {
    Add(String),
    Same(String),
    Remove(String),
    Replace(String, String),
}
