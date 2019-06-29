pub struct QueryMatch<'a> {
    pub column: usize,
    pub value: &'a str
}

impl<'a> QueryMatch<'a> {
    pub fn from(match_index: &(usize, &'a str)) -> QueryMatch<'a> {
        QueryMatch {
            column: match_index.0,
            value: match_index.1
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

