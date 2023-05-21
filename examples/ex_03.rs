trait Animal {
    fn cry(&self);
    fn sleep(&self);
    fn roam(&self);
}

struct Cat { }

impl Animal for Cat {
  fn cry(&self) {
    println!("meow");
  }
  fn sleep(&self) {
    println!("Zzzz");
  }
  fn roam(&self) {
    println!("patter patter pounce");
  }
}

fn main() {
    let cat = Cat {};
    cat.cry();
}
