use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // insert存在则覆盖
    marks.insert("数学", 22);
    marks.insert("英语", 22);

    // haskmap的get返回的是一个Option, 存在键值则为Some否则为None
    match marks.get("数学") {
        Some(mark) => println!("{}", mark),
        None => println!("None"),
    }

    marks.remove("数学");

    for (subject, mark) in &marks {
        println!("{}, {}", subject, mark);
    }

    println!("marks 是否包含英语：{}", marks.contains_key("英语"));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];

    // 引用迭代创建hashmap
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // scores.insert(String::from("test"), 10); //  错误，因为上面创建的是一个引用hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("Blue"), 10);

    // println!("scores: {:?}", scores);
    scores.entry(String::from("Blue")).or_insert(20); // Blue存在，所以不会更新
    scores.entry(String::from("Green")).or_insert(40);
    for (item, score) in &scores {
        println!("{}\t{}", item, score);
    }
}
