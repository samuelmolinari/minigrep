#[derive(Debug)]
pub struct Config {
    pub filepath: String,
    pub query: String
}

impl Config {
    pub fn new(filepath: String, query: String) -> Config {
        Config {
            filepath,
            query
        }
    }

    pub fn parse_args(args: &Vec<String>) -> Config {
        Config::new(args[2].clone(), args[1].clone())
    }
}

#[cfg(test)]
mod test {
    use crate::cmd::Config;

    #[test]
    fn test_parse_args() {
        let arguments: Vec<String> = vec![
            String::from("script"),
            String::from("word"),
            String::from("path/to/file.txt")
        ];
        let config = Config::parse_args(&arguments);

        assert_eq!(config.filepath, "path/to/file.txt");
        assert_eq!(config.query, "word");
    }

    #[test]
    fn test_new() {
        let config = Config::new(
            String::from("path/to/file.txt"),
            String::from("word")
        );

        assert_eq!(config.filepath, "path/to/file.txt");
        assert_eq!(config.query, "word");
    }
}
