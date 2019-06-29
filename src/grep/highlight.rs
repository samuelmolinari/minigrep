use super::query_match::QueryMatch;
use super::line::Line;

const OPEN_TAG: &str = "\u{001b}[1;93m";
const CLOSE_TAG: &str = "\u{001b}[0m";

pub struct Highlight {
    row: usize,
    column: usize,
    value: String
}

impl Highlight {
    pub fn term_output(&self) -> String {
        format!(
            "\u{001b}[1m{}:{}\u{001b}[0m\t{}",
            self.row,
            self.column,
            self.value
        )
    }

    pub fn from(matcher: QueryMatch, line: Line) -> Highlight {
        let mut value = line.value.clone();
        let start = matcher.column;
        let end = start + matcher.len();

        Highlight::tag(&mut value, start, end);

        Highlight {
            row: line.position,
            column: start,
            value
        }
    }

    fn tag(value: &mut String, start: usize, end: usize) {
        value.insert_str(start, OPEN_TAG);
        value.insert_str(end + OPEN_TAG.len(), CLOSE_TAG);
    }
}

#[cfg(test)]
mod test {
    use super::{
        Highlight,
        super:: {
            query_match::QueryMatch,
            line::Line
        }
    };

    #[test]
    fn test_tag() {
        let mut content = String::from("Hello world!");
        Highlight::tag(&mut content, 6, 11);

        assert_eq!(
            content,
            format!(
                "Hello {}world{}!",
                super::OPEN_TAG,
                super::CLOSE_TAG
            )
        )
    }

    #[test]
    fn test_term_output() {
        let highlight = Highlight {
            row: 1,
            column: 6,
            value: String::from("Hello world!")
        };

        assert_eq!(highlight.term_output(), "\u{001b}[1m1:6\u{001b}[0m\tHello world!")
    }

    #[test]
    fn test_from() {
        let matcher = QueryMatch {
            column: 10,
            value: "Hello world😀!"
        };

        let line = Line {
            position: 1,
            value: &String::from("Sam said: Hello world😀!")
        };

        let highlight = Highlight::from(matcher, line);

        assert_eq!(highlight.row, 1);
        assert_eq!(highlight.column, 10);
        assert_eq!(highlight.value, format!("Sam said: {}Hello world😀!{}", super::OPEN_TAG, super::CLOSE_TAG));
    }
}
