use super::Rule;
use crate::error::{Error, ErrorKind};
use pest::iterators::Pair;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReaderMacro {
    Quote,
    Splice,
    Quasiquote,
    Unquote,
    ShortFn,
}

impl ReaderMacro {
    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        pair.as_str()
            .try_into()
            .expect("Pair<Rule> should always return valid reader macros.")
    }
}

impl TryFrom<&str> for ReaderMacro {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let out = match value {
            "'" => ReaderMacro::Quote,
            ";" => ReaderMacro::Splice,
            "~" => ReaderMacro::Quasiquote,
            "," => ReaderMacro::Unquote,
            "|" => ReaderMacro::ShortFn,
            _ => {
                return Err(ErrorKind::Message(format!(
                    "Invalid reader macro '{value}'."
                ))
                .into())
            }
        };

        Ok(out)
    }
}

impl std::fmt::Display for ReaderMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReaderMacro::Quote => "quote",
                ReaderMacro::Splice => "splice",
                ReaderMacro::Quasiquote => "quasiquote",
                ReaderMacro::Unquote => "unquote",
                ReaderMacro::ShortFn => "short-fn",
            }
        )
    }
}
