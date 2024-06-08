pub struct Config {
    pub path: String,
    pub strict: bool,
}

impl Config {
    pub fn new(path: String, strict: bool) -> Config {
        Config { path, strict }
    }
}
pub fn parse_arguments() -> Result<Config, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(String::from("not enought arguments"));
    }

    let path = String::from(args[1].as_str());

    let strict_command = String::from("--strict");

    Ok(Config::new(path, args.contains(&strict_command)))
}

pub fn get_source_code(config: &Config) -> Result<String, std::io::Error> {
    std::fs::read_to_string(&config.path)
}
