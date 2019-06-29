use crate::cmd::Config;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

const OPEN_TAG: &str = "\u{001b}[1;93m";
const CLOSE_TAG: &str = "\u{001b}[0m";

struct QueryMatch<'a> {
    column: usize,
    value: &'a str
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

struct Highlight {
    row: usize,
    column: usize,
    value: String
}

impl Highlight {
    fn println(&self) {
        println!(
            "\u{001b}[1m{}:{}\u{001b}[0m\t{}",
            self.row,
            self.column,
            self.value
        )
    }

    fn from(query_match: QueryMatch, line: Line) -> Highlight {
        let mut value = line.value.clone();
        let start = query_match.column;
        let end = start + query_match.len() + OPEN_TAG.len();

        Highlight::tag(&mut value, start, end);

        Highlight {
            row: line.position,
            column: start,
            value
        }
    }

    fn tag(value: &mut String, start: usize, end: usize) {
        value.insert_str(start, OPEN_TAG);
        value.insert_str(end, CLOSE_TAG);
    }
}

#[derive(Copy, Clone)]
struct Line<'a> {
    position: usize,
    value: &'a String
}

pub fn find(config: Config) -> io::Result<()> {
    let file = File::open(config.filepath)?;

    for (index, value) in BufReader::new(file).lines().enumerate() {
        search_line(
            Line { value: &value?, position: index + 1 },
            &config.query
        );
    }

    Ok(())
}

fn search_line(line: Line, query: &str) {
    let matches = search(line.value, query);

    for a_match in matches.iter() {
        Highlight::from(
            QueryMatch::from(&a_match),
            line
        ).println()
    }
}

fn search<'a>(content: &'a String, query: &str) -> Vec<(usize, &'a str)> {
    content.match_indices(query).collect()
}
