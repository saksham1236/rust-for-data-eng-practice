use rand::seq::IndexedRandom;
use rand::{rng, seq::SliceRandom};
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Debug)]
#[derive(Eq, PartialEq)]

enum Fruit{
    Fig, 
    Other(String)
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = rng();
    let fruits = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Date", 
        "Elderberry",
        "Fig",
        "Fig",
        "Fig",
    ];
    let mut fruit_heap = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_heap.push(Fruit::Fig);
        } else {
            fruit_heap.push(Fruit::Other(fruit.to_string()));
        }
    }
    fruit_heap
}

fn main() {
    
    for _ in 0..20 {
        let salad = generate_fruit_salad();
        println!("Here is the fruit salad {:?}", salad);
    }
}
