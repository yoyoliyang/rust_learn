use std::io;

fn main() {
    let mut input = String::new();

    println!("说点什么吧？");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("说对了！{}", input.to_uppercase());
        }
        Err(e) => {
            println!("出了点问题！{}, {}", e, input);
        }
    }
}
