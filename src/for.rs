fn main() {
    // vec!
    let animals = vec!["Rabbit", "Dog", "Cat"];

    // enumerate 及 iter()
    for (index, a) in animals.iter().enumerate() {
        println!("{}\t{}", index + 1, a)
    }

    for i in 1..10 {
        println!("{}", i);
    }
}
