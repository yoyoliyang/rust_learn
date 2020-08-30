fn main() {
    let s = String::from("ocean hello world");
    // let s = "ocean hello world";
    let n = first_word(&s);
    println!("{}", n);
}

fn first_word(s: &str) -> &str {
    // 对于String或者&str值，都可以使用&str
    let bytes = s.as_bytes();

    // iter
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}, {}", i, &item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
