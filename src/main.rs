use std::env;
use minigrep::grep::config::Config;
use minigrep::grep;
use std::io;

fn main() -> io::Result<()> {
    let config: Config = Config::parse_args(
      &env::args().collect()
    );

    grep::find(config)
}
