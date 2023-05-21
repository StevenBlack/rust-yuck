trait Animal {
    type Base : Animal;
    fn base(&self) -> &Self::Base;
    fn cry(&self) {
      self.base().cry();
    }
    fn sleep(&self) {
      self.base().sleep();
    }
    fn roam(&self) {
      self.base().roam();
    }
}

struct BaseAnimal {}

impl Animal for BaseAnimal {
  type Base = BaseAnimal;
  fn base(&self) -> &BaseAnimal {
    panic!("Never call me here.");
  }
  fn cry(&self) {
    println!("base cry");
  }
  fn sleep(&self) {
    println!("base sleep");
  }
  fn roam(&self) {
    println!("base roam");
  }
}

struct Cat {}

impl Animal for Cat {
  type Base = BaseAnimal;
  fn base(&self) -> &BaseAnimal {
    &BaseAnimal {}
  }
  fn cry(&self) {
    println!("meow");
  }
}

fn main() {
    let cat = Cat {};
    cat.roam();
    cat.cry();
    cat.sleep();
}
