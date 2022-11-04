// ABC's in rust

trait BaseAnimal {
    fn sound(&self) {
        println!("Generic");
    }
}

struct Cat {}

impl BaseAnimal for Cat {
    fn sound(&self) {
        println!("Meow!");
    }
}

struct Dog {}

impl BaseAnimal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}


fn main() {
    let cat: Cat = Cat{};
    cat.sound();

    let dog: Dog = Dog{};
    dog.sound();
}
