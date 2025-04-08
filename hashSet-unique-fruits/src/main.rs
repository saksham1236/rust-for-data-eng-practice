use rand::seq::IndexedRandom;
use::rand::rng;
use::std::collections::HashSet;
use::rand::seq::SliceRandom;

fn generate_fruits() -> String {
    let fruits = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
        "Kiwi",
        "Lemon",
        "Mango",
        "Nectarine",
    ];
    let mut rng = rng();
    fruits.choose(&mut rng).unwrap().to_string()
}

fn create_fruit_set() -> HashSet<String> {
    let mut fruit_set = HashSet::new();
    println!("Adding 100 Random Fruits");
    for _ in 0..10 {
        fruit_set.insert(generate_fruits());
    }
    fruit_set
}
fn main(){
    println!("Adding 100 Random Fruits");
    for _ in 0..20 {
        println!("Number of Fruits generated: {}", create_fruit_set().len())
    }
}
