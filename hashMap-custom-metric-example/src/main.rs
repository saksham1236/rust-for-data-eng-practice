use std::collections::HashMap;

//Would usually use external data sources.
fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("Rust".to_string(), 2010);
    languages.insert("Python".to_string(), 1991);
    languages.insert("Java".to_string(), 1995);
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("C++".to_string(), 1985);
    languages.insert("Go".to_string(), 2009);
    languages.insert("Ruby".to_string(), 1995);
    languages.insert("Swift".to_string(), 2014);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("Kotlin".to_string(), 2011);

    languages
}

//Can have any type of metric or weight
fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = years_active.values().min().unwrap_or(&0);
    let max_year = years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, year) in years_active.iter() {
        let normalized_year = (year-min_year) as f64/(max_year-min_year) as f64;
        let weight = (normalized_year * 100.0) as i32;
        weights.insert(language.to_string(), weight);
    }
    weights
}
fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);
    println!("Weights are: {:?}", weights);//Debug method of printing
    for (language, weight) in weights {
        println!("Language: {language}, Weight: {weight}");
    }
}
