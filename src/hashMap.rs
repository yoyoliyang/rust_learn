use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("数学", 22);
    marks.insert("英语", 22);

    match marks.get("数学") {
        Some(mark) => println!("{}", mark),
        None => println!("None"),
    }

    marks.remove("数学");

    for (subject, mark) in &marks {
        println!("{}, {}", subject, mark);
    }

    println!("marks 是否包含英语：{}", marks.contains_key("英语"));
}
