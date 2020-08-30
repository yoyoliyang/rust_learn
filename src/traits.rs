struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}
// 把trait定义的方法传递给struct中
impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 15,
    };

    person.speak();
    println!("{}", person.can_speak());
}
