mod representative;

use std::{
  collections::HashMap,
  io::{self, Write},
  vec,
};

fn main() {
  index();
  repr();
  piglatin();
  employment();
}

fn index() {
  println!("----- index -----");
  let v = vec![1, 2, 3, 4, 5];

  let item: &i32 = &v[2];
  println!("The element is {}", item);

  match v.get(2) {
    Some(item) => println!("The element is {}", item),
    None => println!("No element."),
  }
}

fn repr() {
  println!("----- repr -----");
  let mut v: Vec<i32> = Vec::new();

  loop {
    let mut buf = String::new();
    io::stdin()
      .read_line(&mut buf)
      .expect("Failed to read line.");

    let a: i32 = match buf.trim().parse() {
      Ok(num) => num,
      Err(_) => break,
    };

    v.push(a);
  }

  v.sort();

  if let Some(mean) = representative::mean(&v) {
    println!("Mean: {}", mean);
  }
  if let Some(median) = representative::median(&v) {
    println!("Median: {}", median);
  }
  if let Some(mode) = representative::mode(&v) {
    println!("Mode: {:?}", mode);
  }
}

fn piglatin() {
  println!("----- pig latin -----");
  let mut buf = String::new();
  io::stdin()
    .read_line(&mut buf)
    .expect("Failed to read line");

  let src = buf.trim();

  match src.chars().next() {
    None => return,
    Some(head) if ['a', 'e', 'i', 'o', 'u'].contains(&head) => {
      print!("{}h", src);
    }
    Some(head) => {
      print!("{}", src.chars().skip(1).collect::<String>());
      print!("{}", head);
    }
  }

  print!("ay\n");
}

fn employment() {
  println!("----- employ -----");
  let mut departments: HashMap<String, Vec<String>> =
    HashMap::new();
  loop {
    print!("> ");
    io::stdout().flush().expect("Cannot flush");
    let mut buf = String::new();
    if let Err(_) = io::stdin().read_line(&mut buf) {
      continue;
    }

    let words: Vec<_> =
      buf.trim().split(" ").filter(|x| *x != "").collect();

    match &words[..] {
      ["add" | "Add" | "ADD", name, "to" | "To" | "TO", dept] => {
        let v =
          departments.entry(String::from(*dept)).or_default();
        v.push(String::from(*name));
      }
      ["list" | "List" | "LIST", "all" | "All" | "ALL"] => {
        for (dept, employees) in &mut departments {
          employees.sort();
          print_dept(dept, employees);
        }
      }
      ["list" | "List" | "LIST", dept] => {
        if let Some(employees) =
          departments.get_mut(&String::from(*dept))
        {
          employees.sort();
          print_dept(dept, employees);
        } else {
          println!("No such department!");
        }
      }
      _ => continue,
    }
  }
}

fn print_dept(dept: &str, employees: &Vec<String>) {
  println!("Department {}[ {} ]", dept, employees.join(", "));
}
