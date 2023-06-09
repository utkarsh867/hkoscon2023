
struct Dog {
    name: String
}

struct Cat {
    name: String
}

enum Animal {
    Dog(Dog),
    Cat(Cat),
}

impl Animal {
    fn make_sound(self) {
        match self {
           Animal::Dog(d) => d.bark(),
           Animal::Cat(c) => c.meow()
        }
    }
}

impl Dog {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name)
        }
    }

    fn bark(&self) {
        println!("Woof woof! I am {}", self.name);
    }
}

impl Cat {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name)
        }
    }

    fn meow(&self) {
        println!("Meow! I am {}", self.name)
    }
}

fn main() {
    let mut my_pets: Vec<Animal> = Vec::new();

    my_pets.push(Animal::Dog(Dog::new("Alexa")));
    my_pets.push(Animal::Cat(Cat::new("Siri")));

    for pet in my_pets {
        pet.make_sound();
    }
}

