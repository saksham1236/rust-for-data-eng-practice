use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    let mut fruits = vec![
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
        "Orange",
        "Papaya",
        "Quince",
        "Raspberry",
        "Strawberry",
        "Tangerine",
        "Ugli",
        "Watermelon",
    ];

    let mut rng = rng();
    fruits.shuffle(&mut rng);

    println!("Fruit Salad:");

    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            println!("{}", item);
        } else {
            println!("{}, ", item);
        }
    }
}
