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

// Extending the API is left as an excercise for the reader
// So let's do this!
struct Fastcat {
    speed: f32,
    base_obj: Cat
}

impl Animal for Fastcat {
    type Base = Cat;
    fn base(&self) -> Option<&Self::Base> {
      Some(&Cat {})
    }
    fn roam(&self) {
      println!("roaming at {}", &self.speed);
    }
    fn dream(&self) {
        &self.base().unwrap().dream();
        println!("they are so slow!")
    }

}

trait EndangeredAnimal : Animal {
    fn is_endangered(&self) -> bool;
}

fn main() {
    let cat = Fastcat {
        speed: 40.5,
        base_obj: Cat {},
    };
    cat.roam();
    cat.cry();
    cat.sleep();
    cat.dream();
}
