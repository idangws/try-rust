struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string() }
    // }
    // fn new<S: Into<String>>(name: S) -> Person {
    //     Person { name: name.into()}
    // }

    fn new<S>(name: S) -> Person 
        where S: Into<String>
    {
        Person { name: name.into()}
    }
}


pub fn main_into() {
    let john = Person::new("John");
    println!("{:?}", john.name);

    let name = "Jane".to_string();

    let _jane = Person::new(name/*.as_ref()*/);

}