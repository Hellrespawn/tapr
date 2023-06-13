use pest::iterators::Pair;

use crate::parser::Rule;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]

// TODO? Add file location to Location?
pub struct Location(usize, usize);

impl Location {
    pub fn new(line_no: usize, col_no: usize) -> Self {
        Self(line_no, col_no)
    }

    pub fn from_pair(pair: &Pair<Rule>) -> Self {
        pair.into()
    }

    pub fn line_no(&self) -> usize {
        self.0
    }

    pub fn col_no(&self) -> usize {
        self.1
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:03}:{:03}]", self.line_no(), self.col_no())
    }
}

impl From<&Pair<'_, Rule>> for Location {
    fn from(pair: &Pair<Rule>) -> Self {
        let (line_no, col_no) = pair.line_col();
        Location::new(line_no, col_no)
    }
}
