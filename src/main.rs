pub mod types;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

use types::parsing::{parse_entry, Stations};

fn main() -> std::io::Result<()> {
    let file = File::open("measurements.txt").expect("file should be readable");
    let mut buf_reader = BufReader::new(file);
    let mut station_list: Stations = Stations(HashMap::new());
    let mut contents = String::new();
    loop {
        let out = buf_reader.read_line(&mut contents);
        if let Ok(size) = out {
            if size < 5 {
                break;
            }
            parse_entry(&contents, size, &mut station_list.0);
        } else {
            break;
        }
        contents.clear();
    }
    let file = std::fs::File::create("baseline_output.txt").expect("opening output file for write");
    let mut buf = std::io::BufWriter::new(file);
    write!(buf, "{}", station_list)?;
    Ok(())
}
