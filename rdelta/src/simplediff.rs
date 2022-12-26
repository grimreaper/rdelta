use crate::addremove::AddRemove;
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

pub fn simplediff<'l>(a: &'l str, b: &'l str) -> impl Iterator<Item = AddRemove> {
    let combined = a.graphemes(true).zip_longest(b.graphemes(true));
    let mut output: Vec<AddRemove> = Vec::new();
    for set in combined {
        match set {
            Both(l, r) if l == r => output.push(AddRemove::Same(l.to_string())),
            Both(l, r) => {
                output.push(AddRemove::Replace(l.to_string(), r.to_string()));
            }
            Left(l) => {
                output.push(AddRemove::Remove(l.to_string()));
            }
            Right(r) => {
                output.push(AddRemove::Add(r.to_string()));
            }
        }
    }
    output.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use super::AddRemove::*;

    #[test]
    fn simplediff_empty_string() {
        let res = simplediff("", "");
        let vec = res.collect_vec();
        assert_eq!(vec, vec![])
    }

    #[test]
    fn simplediff_same_string() {
        let res = simplediff("hel", "hel");
        let vec = res.collect_vec();
        assert_eq!(
            vec,
            vec![
                Same("h".to_string()),
                Same("e".to_string()),
                Same("l".to_string())
            ]
        )
    }

    #[test]
    fn simplediff_add_two() {
        let res = simplediff("h", "hel");
        let vec = res.collect_vec();
        assert_eq!(
            vec,
            vec![
                Same("h".to_string()),
                Add("e".to_string()),
                Add("l".to_string())
            ]
        )
    }

    #[test]
    fn simplediff_remove_two() {
        let res = simplediff("hel", "h");
        let vec = res.collect_vec();
        assert_eq!(
            vec,
            vec![
                Same("h".to_string()),
                Remove("e".to_string()),
                Remove("l".to_string())
            ]
        )
    }
}
