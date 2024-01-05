use cities_json::get_random_cities;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    let filepath: &str = "1rbc.csv";

    // Instanciate a random generator (needs to be outside of the loop) 
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // Open the file (needs to be outside of the loop)
    let mut file: BufWriter<File> = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filepath)
            .unwrap()
    );

    const MIN_RANDOM: f64 = -40.0;
    const MAX_RANDOM: f64 = 51.0;
    for _n in 1..1_000_000_001 {        
        let random_number: f64 = rng.gen_range(MIN_RANDOM..MAX_RANDOM);
        let random_number_rounded: f64 = f64::trunc(random_number* 10.0) / 10.0;
        let line: String = format!("{};{}", get_random_city(), random_number_rounded);

        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn get_random_city() -> String {
    return get_random_cities().name.to_string();
}
