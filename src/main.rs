use std::{env, process};

use blynk::schedule;
use blynk::Config;

fn main() {
    // print banner and bootstrap!
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("blynk 😉 v{}", VERSION);
    println!("Official Release Site: https://github.com/BirnadinErick/blynk/releases");
    println!("by Birnadin Erick[https://www.github.com/BirnadinErick]");
    println!("bootstrapping...");

    // config parsing
    let config = Config::new(env::args()).unwrap_or_else(|_| {
        eprintln!("Sorry, there was a problem parsing the args given!");
        eprintln!("Given args are:-\n{:?}", env::args().collect::<String>());
        process::exit(1);
    });

    let duration: String;
    if config.get_duration() > 60 {
        duration = format!("{} minutes", config.get_duration() / 60);
    } else {
        duration = format!("{} seconds", config.get_duration());
    }

    println!(
        "Configuration:-\n\tduration: {}\n\tmessage: {}",
        duration,
        config.get_message()
    );

    // register exit handler and end bootstrap
    ctrlc::set_handler(move || {
        println!("Exiting...");
        println!("See ya later 👋, have a nice day 🎉");
        process::exit(0);
    })
    .expect("Error setting Exit handler!");

    println!("Done bootstrapping!");
    println!("Press CTRL/CMD + C to stop blynk");

    // schedule and loop
    loop {
        schedule::schedule(&config);
    }
}
