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
    fn foo(&self) {
      println!("foo!!");
    }
}

struct Cat {}

impl Animal for Cat {
  type Base = Cat;
  fn base(&self) -> &Cat {
    &Cat {} // My base type is... myself?
  }
  fn cry(&self) {
    println!("meow");
  }
}

fn main() {
    let cat = Cat {};
    cat.cry();
    cat.foo();
}
