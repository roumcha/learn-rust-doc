fn main() {}

#[allow(unused)]
fn user() {
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };
  user1.email = String::from("anotheremail@examlple.com");

  fn build_user(email: String, username: String) -> User {
    User {
      email,
      username,
      active: true,
      sign_in_count: 1,
    }
  }

  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  let User {
    email, active: a, ..
  } = user2;
}

#[allow(unused)]
fn color_and_point() {
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  let Color(r, g, b) = black;
}

#[allow(unused)]
fn matching() {
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };
  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    _ => (),
  }
}

#[allow(unused)]
fn matching2() {
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    // コード提案でアームを生成できた。便利。
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => println!(
      "Move in the x direction {} and in the y direction {}",
      x, y
    ),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r, g, b
      )
    }
  }
  // other_function(msg); // 不可。Write(String) の所有権が移動してしまう
}

#[allow(unused)]
fn matching_ref() {
  struct Point {
    x: i32,
    y: i32,
  }

  let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
  ];

  let sum_of_squares: i32 =
    points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}
