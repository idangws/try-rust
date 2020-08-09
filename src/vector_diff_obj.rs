trait Animal {
    fn create(name: &'static str) -> Self where Self: Sized;

    fn name(&self) -> &'static str;
    
    fn talk(&self) {
        println!("{} cannot talk ", self.name())
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{ name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} say hello", self.name())
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} meowing", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat)
}

pub fn main_vctr_diff_obj() {
    let mut creatures = Vec::new();

    creatures.push(Creature::Human(
        Human { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Fluffy" }
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    let mut animals:Vec<Box<dyn Animal>> = Vec::new();

    animals.push(
        Box::new(Human{ name: "John" })
    );
    animals.push(
        Box::new(Cat{ name: "Meme" })
    );

    for a in animals.iter() {
        a.talk();
    }
}