#[derive(Clone, Copy)]
struct Goose  {
    name: &'static str,
    age: u8,
}

struct Ant {
    name: &'static str,
    num_legs: u8,
    age: u8,
}

trait Animal {

    fn name(&self) -> &'static str {
        self.name
    }
    fn age(&self) -> u8 {
        self.age
    }

}

impl Goose {
       fn new(name: &'static str, age: u8) -> Goose {
           Goose {name: name, age: age}
       }
       //
       // fn name(self) -> &'static str {
       //     self.name
       // }

       fn age(self) -> u8{
           self.age
       }
}

fn main() {
    let a: Goose = Goose::new("Bob", 32);

    let a_name = a.name();
    let a_age = a.age();

    println!("{}",a_name);
    println!("{}", a_age);
}
