fn main() {
    println!(
        "{}",
        match get_name("liyang") {
            Some(o) => o,
            None => "None",
        }
    );
}

fn get_name(name: &str) -> Option<&str> {
    match name {
        "liyang" => Some("ok"),
        "oceanlee" => Some("yes"),
        _ => None,
    }
}
