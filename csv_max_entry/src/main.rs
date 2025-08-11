use core::cmp::Reverse;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Record {
    key: String,
    value: i32,
}

fn read_csv_to_vec(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_path(path)?;

    let mut entries = Vec::new();

    for result in csv_reader.deserialize() {
        let record: Record = result?;
        entries.push(record);
    }

    Ok(entries)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut entries = read_csv_to_vec("keywords.csv")?;
    // println!("CSV entries (unsorted):");
    // for Record { key, value } in &entries {
    //     print!("{} -> {};", key, value);
    // }

    if let Some(Record {
        key: max_key,
        value: max_value,
    }) = entries.iter().max_by_key(|entry| entry.value)
    {
        println!("Entry with max value: {} -> {}", max_key, max_value);
    } else {
        println!("No entries found.");
    }

    entries.sort_by_key(|entry| Reverse(entry.value));

    println!("Keywords sorted by their value (highest to lowest):");
    for Record { key, value } in &entries {
        print!("{} -> {};", key, value);
    }

    Ok(())
}
