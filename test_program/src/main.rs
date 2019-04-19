mod foobar;
use crate::foobar::Funny; // Funny is both the name of the derive macro, as well as the trait

#[derive(Funny)]
struct Thing {
    x: u32
}

struct OtherThing {
    y: u32
}

impl Funny for OtherThing {
    fn joke(&self) { println!("The thing about Arsenal is they always try to walk it in."); }
}

fn main() {
    let thing = Thing { x: 100 };
    let other_thing = OtherThing { y: 40 };
    
    thing.joke();
    other_thing.joke();
}
