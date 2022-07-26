use std::thread;
use std::time::Duration;

use crate::config::Config;
use crate::notify::notify_user;

pub fn schedule(config: &Config) {
    thread::sleep(Duration::from_secs(config.get_duration()));
    notify_user(config.get_message());
}

#[cfg(test)]
mod tests {

    #[test]
    fn schedule_for_ten_seconds() {
        use super::*;
        use std::time::Instant;
        let args: Vec<String> = vec![
            String::from("fg"),
            String::from("10"),
            String::from("Hello"),
        ];

        let config = Config::new(args.iter().map(|s| s.to_string())).unwrap();

        let now = Instant::now();

        schedule(&config);

        assert_eq!(
            now.elapsed().as_secs(),
            Duration::from_secs(config.get_duration()).as_secs()
        );
    }
}
