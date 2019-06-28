use crate::cmd::Config;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn find(config: Config) -> io::Result<()> {
    let open_tag = "\u{001b}[31m";
    let close_tag = "\u{001b}[0m";
    let extra_chars_size = open_tag.len() + close_tag.len();

    let size = config.query.len();
    let content = get_content(&config.filepath)?;
    let mut content_matched = content.clone();
    let matches = search(&content, &config.query);

    for (i, a_match) in matches.iter().enumerate() {
        let extra = extra_chars_size * i;
        let range = (
            a_match.0 + extra,
            a_match.0 + extra + size + open_tag.len()
        );
        content_matched.insert_str(range.0, open_tag);
        content_matched.insert_str(range.1, close_tag);
    }

    println!("{}", content_matched);

    Ok(())
}

fn search<'a>(content: &'a String, query: &str) -> Vec<(usize, &'a str)> {
    content.match_indices(query).collect()
}

fn get_content(filepath: &str) -> io::Result<String> {
    let mut content: String = String::new();

    File::open(filepath)?
        .read_to_string(&mut content)?;

    Ok(content)
}
