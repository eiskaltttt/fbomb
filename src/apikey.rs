use std::io;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Apikey {
    key: String,
}

pub(crate) fn apikey() -> Result<()> {
    // Some data structure.

    println!("{}", "请输入apikey".yellow());

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");

    let apikey = Apikey {
        key: key.trim().to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string_pretty(&apikey)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j.blue().bold());
    let _ = std::fs::write("apikey.json", j);

    Ok(())
}