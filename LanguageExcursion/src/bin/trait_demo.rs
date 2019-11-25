extern crate rand;
use rand::Rng;

// #[derive(Clone, Copy)]

trait Animal {

    fn introduce(&self) {
        println!("Type {} is named {} with weight and age: {} and {}, respectively", &self.type_of(), *self.name(), self.weight(), self.age());
    }

    fn name(&self) -> &String;

    fn type_of(&self) -> String;

    fn weight(&self) -> i32;

    fn age(&self) -> u8;

}

struct Chicken {
    name: String,
    age: u8,
    weight: i32,
}

impl Chicken{
    fn eat(&mut self, weight: i32) {
            self.weight += (weight - 50);
    }
    fn new(name: String, age: u8, weight: i32) -> Chicken {
        Chicken {name: name, age: age, weight:weight}
    }
}

impl Animal for Chicken {
    fn type_of(&self) -> String {
        "Chicken".to_string()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn weight(&self) -> i32 {
        self.weight
    }

    fn age(&self) -> u8 {
        self.age
    }
}

// Ant Object

enum AntRole{
    Queen,
    Worker,
    Guard,
    Harvester,
}

struct Ant {
    name: String,
    role: AntRole,
    weight: i32,
    age: u8,
}

impl Ant {
    fn new(name: String, role: AntRole, weight: i32, age: u8) -> Ant {
        Ant {name: name, role: role, weight: weight, age: age}
    }
}

impl Animal for Ant {
    fn type_of(&self) -> String {
        "Ant".to_string()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn weight(&self) -> i32 {
        self.weight
    }

    fn age(&self) -> u8 {
        self.age
    }
}

fn main() {
    let mut flock = [Chicken::new("Gloria".to_string(), 3, 0), Chicken {name: "Mallory".to_string(), age:6, weight: -10},
        Chicken {name: "Clara".to_string(), age:1, weight: 100}];


    let mut rng = rand::thread_rng();
    let mut ant_hill = vec![Ant::new("0".to_string(), AntRole::Queen, 100, 0)];

    for i in 1..200 {
        let random: i32 = rng.gen_range(0,250);
        let name: String = i.to_string();
        let role: AntRole;

        if random < 20 {
            role = AntRole::Harvester;
        }
        else if random < 45 {
            role = AntRole::Worker;
        }
        else if random < 200 {
            role = AntRole::Guard;
        }
        else {
            role = AntRole::Queen;
        }
        ant_hill.push(Ant::new(name, role, random, 0))
        // ant_hill.push(Ant {name:name, role:role, weight:random, age:0});
    }

    for i in 0..3 {
        let ant : Ant = ant_hill.pop().unwrap();
        ant.introduce();
        flock[0].introduce();
        flock[0].eat(ant.weight());
        flock[0].introduce();
    }

}
