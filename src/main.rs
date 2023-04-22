use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    let mut records: Vec<Vec<String>> = csv_reader
        .records()
        .map(|record| {
            record
                .expect("Failed to parse CSV record")
                .into_iter()
                .map(String::from)
                .collect()
        })
        .collect();

    if !records.iter().all(|record| record.len() == 2) {
        eprintln!("Error: CSV file must have 2 columns");
        process::exit(1);
    }

    let mut rng = rand::thread_rng();

    loop {
        records.shuffle(&mut rng);

        for record in &records {
            print!("{}", record[0]);
            io::stdout().flush().unwrap();
            wait_for_enter();

            print!("{}", record[1]);
            io::stdout().flush().unwrap();
            wait_for_enter();

            println!("---");
        }
    }
}

fn wait_for_enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if !input.trim().is_empty() {
        process::exit(0);
    }
}
