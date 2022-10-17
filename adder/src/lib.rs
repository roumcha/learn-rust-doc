#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exploration() {
    assert_eq!(2 + 2, 4);
  }

  // #[test]
  // fn another() {
  //   panic!("Make this test fail");
  // }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      height: 7,
      width: 8,
    };
    let smaller = Rectangle {
      height: 1,
      width: 5,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cant_hold_larger() {
    let larger = Rectangle {
      height: 7,
      width: 8,
    };
    let smaller = Rectangle {
      height: 1,
      width: 5,
    };

    assert!(!smaller.can_hold(&larger));
  }
}
