pub struct Config {
    duration: u64,
    message: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // jump over the cmd given in the first index
        args.next();

        let duration = match args.next() {
            Some(t) => match t.parse::<u64>() {
                Ok(i) => i,
                Err(_) => return Err("Given duration argument is not a digit!"),
            },
            None => 15 * 60 as u64,
        };

        let message = match args.next() {
            Some(m) => m,
            None => String::from("Blink 😉 ryt fuqing NOW!"),
        };

        Ok(Config { duration, message })
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_valid() {
        let args: Vec<String> = vec![
            String::from("fg"),
            String::from("20"),
            String::from("Hello"),
        ];

        let config = Config::new(args.iter().map(|s| s.to_string())).unwrap();

        assert_eq!(config.get_duration(), 20);
        assert_eq!(config.get_message(), String::from("Hello"));
    }

    #[test]
    #[should_panic]
    fn parse_args_invalid() {
        let args: Vec<String> = vec![
            String::from("fs"),
            String::from("11de1"),
            String::from("Hello"),
        ];

        match Config::new(args.iter().map(|s| s.to_string())) {
            Ok(c) => c,
            Err(_e) => panic!("{}", _e),
        };
    }
}
