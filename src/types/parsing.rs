use super::nums::Value;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct Station {
    min: Value,
    mean: Value,
    max: Value,
    count: i32,
}

#[derive(Debug, Clone)]
pub struct Entry {
    station: String,
    temp: Value,
}

pub struct Stations(pub HashMap<String, Station>);

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
            self.min.to_float() / 10.0,
            self.mean.to_float() / 10.0,
            self.max.to_float() / 10.0,
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

pub fn parse_entry<'a>(value: &String, size: usize, station_list: &mut HashMap<String, Station>) {
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
        s.mean = (s.mean * s.count + entry.temp) / s.count + 1;
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
        s.count += 1;
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
