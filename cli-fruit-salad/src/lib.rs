use rand::{rng, seq::SliceRandom};

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits  = vec![
        String::from("Apple"),
        String::from("Banana"),
        String::from("Cherry"),
        String::from("Date"),
        String::from("Elderberry"),
        String::from("Fig"),
        String::from("Grape"),
        String::from("Honeydew"),
        String::from("Kiwi"),
    ];
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits[..num_fruits].to_vec()
}

