use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(Debug, Clone, Copy)]
struct Station {
    min: i32,
    mean: i32,
    max: i32,
    count: i32,
}

#[derive(Debug, Clone)]
struct Entry {
    station: String,
    temp: i16,
}

struct Stations(HashMap<String, Station>);

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.station, self.temp)
    }
}

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            Into::<f64>::into(self.min) / 10.0,
            Into::<f64>::into(self.mean) / 10.0,
            Into::<f64>::into(self.max) / 10.0,
        )
    }
}

impl Display for Stations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.0.iter().peekable();
        while let Some((key, value)) = iter.next() {
            write!(f, "{}={}", key, value)?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

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
    let file = std::fs::File::create("output.txt").expect("opening output file for write");
    let mut buf = std::io::BufWriter::new(file);
    write!(buf, "{}", station_list)
}

fn parse_entry<'a>(value: &String, size: usize, station_list: &mut HashMap<String, Station>) {
    let temp = value.rfind(';').unwrap();
    let temp2 = value.rfind('\n').unwrap();
    let entry = Entry {
        station: value[0..temp].to_string(),
        temp: format!(
            "{}{}",
            value[temp + 1..size - 3].to_string(),
            value[size - 2..temp2].to_string()
        )
        .parse()
        .unwrap(),
    };
    if let Some(s) = station_list.get_mut(&entry.station) {
        s.mean = (s.mean * s.count + Into::<i32>::into(entry.temp)) / s.count + 1;
        s.max = if s.max < entry.temp.into() {
            entry.temp.into()
        } else {
            s.max
        };
        s.min = if s.min > entry.temp.into() {
            entry.temp.into()
        } else {
            s.min
        };
    } else {
        station_list.insert(
            entry.station,
            Station {
                min: entry.temp.into(),
                mean: entry.temp.into(),
                max: entry.temp.into(),
                count: 1,
            },
        );
    }
}
