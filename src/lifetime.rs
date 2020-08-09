struct Person<'a> {
    name: &'a str
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi my name {}", self.name)
    }
}

pub fn main_lifetime() {
    let person = Person { name: "Dimitri" };
    person.talk();
}