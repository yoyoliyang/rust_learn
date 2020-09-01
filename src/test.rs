fn main() {
    let str1 = String::from("hello,world");
    let str2 = "xyz";

    let result = longest(&str1, str2);

    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
