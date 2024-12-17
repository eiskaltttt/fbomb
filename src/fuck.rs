use std::io;

pub fn fuck(f: [&str; 8]) -> String {
    println!("{:#?}",f);
    println!("输入轰炸方法编号");
    let mut method = String::new();
    io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");

    let damm : usize = method.trim().parse().unwrap();

    let sb = f[damm];
    return sb.to_string();
}