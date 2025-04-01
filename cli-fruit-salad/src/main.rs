use cli_fruit_salad::create_fruit_salad;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "cli-fruit-salad", version = "1.0", author = "Your Name", about = "A simple CLI fruit salad generator")]

struct Opts {
    /// Number of fruits to include in the salad
    #[clap(short, long, default_value = "5")]
    n: usize,
}
fn main() {
    let opts: Opts = Opts::parse();

    //Get the num_fruits requested
    let num_fruits: usize = opts.n;

    //Create the fruit salad;
    create_fruit_salad(num_fruits);

    //Print the salad in human readable format
    println!(
        "Create fruit salad with {} fruits: {:?}", num_fruits, create_fruit_salad(num_fruits)
    )
}
