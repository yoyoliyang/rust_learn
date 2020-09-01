use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;

fn main() {
    // reading a file
    let file_name = "info.text".to_string();
    let mut file = File::open(&file_name).expect("Can't open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("打开错误！");

    match read_file_content(&file_name) {
        Ok(data) => println!("读取文本为：{}", data),
        Err(e) => println!("{}", e),
    }

    // println!("{}", read_file_content(&file_name));

    let mut file1 = File::create("output.text").expect("error create file");

    file1
        .write_all(b"Hello world!\n")
        .expect("Cannot write to the file");

    /*    match file2 {
            Ok(data) => {
                println!("{}", data);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    */
}

fn read_file_content(name: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    // ? 问号是传播错误的简写
    let file2 = File::open(name)?.read_to_string(&mut s);
    Ok(s)
}
