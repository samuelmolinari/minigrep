pub mod query_match;
pub mod highlight;
pub mod line;
pub mod config;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use {
    config::Config,
    query_match::QueryMatch,
    highlight::Highlight,
    line::Line
};

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
        let output = Highlight::from(
            QueryMatch::from(&a_match),
            line
        ).term_output();

        println!("{}", output);
    }
}

fn search<'a>(content: &'a String, query: &str) -> Vec<(usize, &'a str)> {
    content.match_indices(query).collect()
}
