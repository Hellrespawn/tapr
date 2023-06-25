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
    pub fn from_pairs(pairs: Vec<Pair<Rule>>) -> Vec<Self> {
        pairs
            .into_iter()
            .map(|p| {
                p.as_str().try_into().expect(
                    "Pair<Rule> should always return valid reader macros.",
                )
            })
            .collect()
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
                return Err(
                    ErrorKind::InvalidReaderMacro(value.to_owned()).into()
                )
            }
        };

        Ok(out)
    }
}
