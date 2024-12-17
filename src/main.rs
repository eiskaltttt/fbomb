use colored::*;
use std::io;

//const  apikey: &str = "7BUE5YGAS0TW1VC";

mod cc;
mod fuck;
mod apikey;

fn main() {
    println!("{}", "最终秩序轰炸机!（该项目已废弃 无法使用 因为攻击平台倒闭了）".red().bold());
    println!("{}", "1.DdoS".blue());
    println!("{}", "2.CC".purple().magenta());
    println!("{}", "3.立即轰炸".green());
    println!("{}", "4.设置apikey".yellow());
    println!("请选择:");


    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    if a.trim() == "1" {
        println!("DdoS");
        ddos();
    } else if a.trim() == "2" {
        println!("CC");
        cc::ccattack();
    } else if a.trim() == "3"{
        println!("即将轰炸");
    } else {
        let _ = apikey::apikey();

    }
}

fn ddos() {
    println!("输入敌方IP");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read line");
    println!("输入敌方端口");
    let mut port = String::new();
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to read line");
    println!("输入轰炸方法");
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
}
