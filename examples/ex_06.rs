trait Animal {
    type Base : Animal;
    fn base(&self) -> Option<&Self::Base> {
      None
    }
    fn cry(&self) {
      if let Some(b) = self.base() {
        b.cry();
      } else {
        todo!("cry() method not yet implemented");
      }
    }
    fn sleep(&self) {
        if let Some(b) = self.base() {
          b.sleep();
        } else {
          todo!("sleep() method not yet implemented");
        }
    }
    fn roam(&self) {
        if let Some(b) = self.base() {
          b.roam();
        } else {
          todo!("roam() method not yet implemented");
        }
    }
    fn dream(&self) {
        if let Some(b) = self.base() {
          b.dream();
        } else {
          println!("dreaming!");
        }
    }
  }
struct BaseAnimal {}

impl Animal for BaseAnimal {
  type Base = BaseAnimal;
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
  fn base(&self) -> Option<&Self::Base> {
    Some(&BaseAnimal {})
  }
  fn cry(&self) {
    println!("meow");
  }
  fn dream(&self) {
    println!("mmmm mice!");
  }
}

fn main() {
    let cat = Cat {};
    cat.roam();
    cat.cry();
    cat.sleep();
    cat.dream();
}
