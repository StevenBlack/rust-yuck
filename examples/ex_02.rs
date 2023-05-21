trait Animal {
    fn cry();
    fn sleep();
    fn roam();
}

struct Cat {}

impl Animal for Cat {
    fn cry() {
      println!("meow");
    }
    fn sleep() {
      println!("Zzzz");
    }
    fn roam() {
      println!("patter patter pounce");
    }
}

struct Bird {}

impl Animal for Bird {
  fn cry() {
    println!("chirp");
  }
  fn sleep() {
    println!("Zzzz");
  }
  fn roam() {
    println!("flit flit flurp");
  }
}

fn cry_to_sleep<T: Animal>() {
    T::cry();
    T::cry();
    T::sleep();
}

fn main() {
    Cat::cry();
    Bird::roam();
    cry_to_sleep::<Bird>();
    cry_to_sleep::<Cat>();
}
