fn main() {
    let b = Bird::new();
    b.fly();

    let g = Dog::born();
    g.cry();
}

struct Bird;

struct Dog {}

impl Bird {
    fn new() -> Self {
        Bird
    }
    fn fly(&self) {
        println!("i am flying");
    }
}



impl Dog {
    fn born() -> Self {
        Dog {}
    }

    fn cry(&self) {
        println!("crying");
    }
}
