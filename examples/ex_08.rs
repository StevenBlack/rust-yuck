trait Animal {
    fn cry(&self);
    fn sleep(&self);
    fn roam(&self);
}

struct Cat {}

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

struct Bird {}

impl Animal for Bird {
  fn cry(&self) {
    println!("chirp");
  }
  fn sleep(&self) {
    println!("Zzzz");
  }
  fn roam(&self) {
    println!("flit flit flurp");
  }
}

fn cry_to_sleep_dynamic(animal: &dyn Animal) {
    animal.cry();
    animal.cry();
    animal.sleep();
    // animal.dream();
  }

fn main() {
    let cat = Cat {};
    let bird = Bird{};
    cry_to_sleep_dynamic(&cat);
    cry_to_sleep_dynamic(&bird);
}
