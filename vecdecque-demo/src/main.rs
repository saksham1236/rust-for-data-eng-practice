use std::{collections::VecDeque, os::windows::thread};
use rand::{rng, seq::SliceRandom};

//Vec is double-ended queue, which means it can be used to push and pop elements from both ends.
fn main() {
    let mut fruits:VecDeque<&str> = VecDeque::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Cherry");

    // Shuffle the fruits
    let mut rng = rng();
    let mut fruits = Vec::from(fruits);
    fruits.shuffle(&mut rng);

    //Convert Vec back to VecDeque
    let mut fruits:VecDeque<&str> = VecDeque::from(fruits);

    fruits.push_front("Pomegranate");
    fruits.push_back("Mango");

    print!("{:?}", fruits);

}
