/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and witse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
os a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use std::collections::LinkedList;
use rand::{rng, seq::SliceRandom};

fn main() {
    let mut fruits:LinkedList<&str> = LinkedList::new();

    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Cherry");
    
    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Shuffle the fruits
    let mut rng = rng();
    let mut fruits:Vec<&str> = Vec::from_iter(fruits);

    fruits.shuffle(&mut rng);

    let mut fruits:LinkedList<&str> = LinkedList::from_iter(fruits);

    fruits.push_front("Pomegranate");
    fruits.push_back("Fig");

    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{},", item);
        } else {
            println!("{},this not a fruit", item);
        }
    }

}
