
use std::sync::{Arc, Mutex};

struct Boxed(String);

impl Boxed {
  fn consume(mut a: &Self) -> () {
    a = a;
  }
  fn inc(&mut self) -> &String {
    // self.0 += 1;
    &self.0
  }
}

fn test() {
  let r: &String ={ Boxed("5".into()).inc()};
  ()
}

fn main() {
    let a = Arc::new(Mutex::new(5));
    let mut x = (*a).lock().unwrap();
    *x += 1;
    drop(x);
    let x = a.lock().unwrap();
    assert!(*x == 6);
}

// fn main1() {
//     let mut x = 5;
//     let r1 = &mut x;
//     let r2 = &mut x;
//     println!("{} {}", r1, r2);
// }

// fn main2() {
//     let s = String::from("hello");
//     let s2 = s;
//     drop(s2);
//     println!("{}", s);
// }


// fn say_what() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn main3() {
//     let mut x = 5;
//     let r1 = &x;
//     let r2 = &mut x;
//     println!("{}", r1);
// }


// fn main4() {
//     let mut x = 5;
//     {
//         let r1 = &mut x;
//         *r1 += 1;
//     } 
//     let r2 = &mut x;
//     *r2 += 1;
//     println!("{}", x);
// }