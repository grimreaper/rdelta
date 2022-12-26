use std::ops::Deref;
use colored::Colorize;
use crate::addremove::AddRemove;

pub fn coloroutputdiff<'a>(datum: impl Iterator<Item = AddRemove>) -> String {
    let mut out = String::new();
    for item in datum {
        match item {
            AddRemove::Add(s) => {
                out.push_str(s.green().to_string().as_str());
            }
            AddRemove::Remove(s) => {
                out.push_str(s.red().to_string().as_str());
            }
            AddRemove::Same(s) => {
                out.push_str(s.deref());
            }
            AddRemove::Replace(l, r) => {
                out.push_str(l.red().to_string().as_str());
                out.push_str(r.green().to_string().as_str());
            }
        }
    }
    out
}

pub fn simpleoutput<'a>(datum: impl Iterator<Item = AddRemove>) -> String {
    let mut out = String::new();
    for item in datum {
        match item {
            AddRemove::Add(s) => {
                out.push_str(format!("+{}", s).as_str());
            }
            AddRemove::Remove(s) => {
                out.push_str(format!("-{}", s).as_str());
            }
            AddRemove::Same(s) => {
                out.push_str(s.as_str());
            }
            AddRemove::Replace(rem, add) => {
                out.push_str(format!("-{}+{}", rem, add).as_str());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use super::AddRemove::*;

    #[test]
    fn coloroutputdiff_empty_string() {
        let output = coloroutputdiff(vec!().into_iter());
        assert_eq!(output, "")
    }

    #[test]
    fn coloroutput_same_string() {
        let output = coloroutputdiff(vec!(Same("a".to_owned())).into_iter());
        assert_eq!(output, "a")
    }


    #[test]
    fn simpleoutput_empty_string() {
        let output = simpleoutput(vec!().into_iter());
        assert_eq!(output, "")
    }

    #[test]
    fn simpleoutput_same_string() {
        let output = simpleoutput(vec!(Same("a".to_owned())).into_iter());
        assert_eq!(output, "a")
    }

    #[test]
    fn simpleoutput_add_string() {
        let output = simpleoutput(vec!(Add("a".to_owned())).into_iter());
        assert_eq!(output, "+a")
    }

    #[test]
    fn simpleoutput_remove_string() {
        let output = simpleoutput(vec!(Remove("a".to_owned())).into_iter());
        assert_eq!(output, "-a")
    }


}
