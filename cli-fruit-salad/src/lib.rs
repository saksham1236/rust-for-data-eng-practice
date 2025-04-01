use rand::{rng, seq::SliceRandom};

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits  = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
        "Fig".to_string(),
        "Grape".to_string(),
        "Honeydew".to_string(),
        "Kiwi".to_string(),
    ];
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits[..num_fruits].to_vec()   
}

