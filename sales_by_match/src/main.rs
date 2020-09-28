use std::io::{stdin};
use std::collections::HashMap;

fn main() {
    let mut _count = String::new();
    let mut s = String::new();

    stdin().read_line(&mut _count).expect("err");
    stdin().read_line(&mut s).expect("err");

    let v: Vec<&str> = s.trim_end().split(" ").collect();

    let mut h = HashMap::new();
    let mut result = 0;
    for s in &v {
        let v = h.remove(s);

        match v {
            Some(_n) => { result += 1; }
            None => { h.insert(s, 1); }
        };
    }

    println!("{}", result);
}
