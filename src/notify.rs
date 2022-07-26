use notify_rust::{Notification, Timeout};

pub fn notify_user(s: &str) {
    let _ = Notification::new()
        .summary(s)
        .body("Your time is up ⏲, please take a break and keep blinking for a moment")
        .appname("blynk 😉")
        .timeout(Timeout::Never)
        .show();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notify_check() {
        assert_eq!(notify_user("Blink ryt fuqing NOW!"), ());
    }
}
