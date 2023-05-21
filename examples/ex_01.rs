#[derive(Debug)]
struct Mouse { // a "class"
  age: u8 // an unsigned 8-bit,
          // a byte, or a "datum"
}

impl Mouse {
  fn is_old(&self) -> bool { // a "method"
    self.age >= 2
  }
}

#[derive(Debug)]
pub struct Rat {
    age: u8 // Field is not public so Rat
            // can't be constructed and age
            // can't be accessed externally.
}

impl Rat {
    pub fn new(years_old: u8) -> Rat { // a "static method"
      return Rat { age: years_old };
    }

    pub fn is_old(&self) -> bool { // an "instance method"
      self.age >= 3
    }
  }

pub trait Old { // an "interface"
  fn is_old(&self) -> bool;
}

fn main() {

    let mouse = Mouse { age: 1 }; // an "object"
    let rat = Rat::new(3);

    println!("mouse is old? {}", mouse.is_old());
    println!("rat is old? {}", rat.is_old());
}
