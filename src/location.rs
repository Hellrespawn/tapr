#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location(usize, usize);

impl Location {
    pub fn new(line_no: usize, col_no: usize) -> Self {
        Self(line_no, col_no)
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
