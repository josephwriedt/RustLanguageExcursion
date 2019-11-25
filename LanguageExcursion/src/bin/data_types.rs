extern crate rand;
use rand::Rng;


fn int_demo(){
    // 8, 16, 32, 64, 128, arch bit sizes

    let mut big_int: usize = 1_000_000;
    println!("Current number is {}", big_int);
    big_int += 1;
    println!("Current number is {}", big_int);
}

fn float_demo(){
    //floats are either f32 or f64 (f64 is default because of modern day computers)
    let mut float_num: f32 = 2.68;

    fn print_float(float_addr: &f32) {
        println!("The float that is addressed to is: {}", *float_addr);
    }

    print_float(&float_num);

    fn change_float(float_addr: &mut f32){
        *float_addr += 1.0;
    }
    change_float(&mut float_num);
    println!("The float now is: {}", float_num);
}

fn string_demo(){
    // Two types of string declarations

    // Variable sized string using vectors of bytes
    let mut a = String::new();
    for i in 0..10 {
        a.push(char::from(i));
        a.push_str(", ");
        a.pop();
    }

    // A reference to a string allocated in read only memory
    let b: &'static str = "Hello world";
    println!("({}) and ({})",a,b)
}

struct Chicken {
    name: &'static str,
    age: u8,
    weight: i32,
}

impl Chicken{
    fn eat(&mut self, weight: u32){
            self.weight += (weight as i32 - 20);
    }
}

enum AntRole{
    Queen,
    Worker,
    Guard,
    Harvester,
}

struct Ant {
    name: u32,
    role: AntRole,
    weight: u32,
    age: u8,
}


fn array_demo(){
    let flock = [Chicken {name: "Gloria", age:3, weight: 0}, Chicken {name: "Mallory", age:6, weight: -10},
        Chicken {name: "Clara", age:1, weight: 100}];


    let mut rng = rand::thread_rng();
    let mut ant_hill: Vec<Ant> = vec![Ant {name: 0, role: AntRole::Queen, weight: 100, age: 0}];

    for i in 1..200 {
        let random = rng.gen_range(0,250);

        if random < 20 {
            ant_hill.push(Ant {name:i, role: AntRole::Harvester, weight: random, age:0});
        }
        else if random < 45 {
            ant_hill.push(Ant {name:i, role: AntRole::Worker, weight: random, age:0});
        }
        else if random < 200 {
            ant_hill.push(Ant {name: i, role: AntRole::Guard, weight: random, age:0});
        }
        else {
            ant_hill.push(Ant {name: i, role: AntRole::Queen, weight: random, age:0})
        }
    }

    // Tuples are collections of potentially many different types
    // Elements are accessed from the tuple by .index_num
    let hodgepodge_animals = (Chicken {name: "Hope", age:4, weight: 0}, Ant {name: 0, role: AntRole::Worker, weight:24, age:0});

}

fn main() {

// int_demo();
// float_demo();
// string_demo();
// array_demo();

}
