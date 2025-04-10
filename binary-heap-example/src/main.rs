use rand::{rng, seq::SliceRandom};
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]

enum Fruit{
    Fig, 
    Other(String)
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Greater,

        }
    }
}
fn main() {
    println!("Hello, world!");
}
