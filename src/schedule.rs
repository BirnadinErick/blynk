use std::thread;
use std::time::{Duration, Instant};

use crate::notify::notify_user;

pub fn schedule(t: u64) {
    let duration = Duration::from_secs(t);
    thread::sleep(duration);

    notify_user("Blink 😉 ryt fuqing now!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn schedule_for_ten_seconds() {
        use super::*;

        let now = Instant::now();

        schedule(10);

        assert_eq!(now.elapsed().as_secs(), Duration::from_secs(10).as_secs());
    }
}
