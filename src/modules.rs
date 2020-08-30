// 外部调用方式
mod t;

// 内部定义方式
mod t1 {
    pub fn print_message() {
        println!("hello");
    }
}

fn main() {
    // l 此时为&str
    // let l = "liyang";
    let l = String::from("liyang");
    let r = t::print_message(l);
    println!("{}", r);

    t1::print_message();
}
