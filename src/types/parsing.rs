use super::nums::Value;
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex, RwLock},
};

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

pub struct Stations(pub HashMap<String, Arc<Mutex<Station>>>);

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.station, self.temp)
    }
}

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.min > self.mean {
            panic!("Min {} greater than mean {}", self.min, self.mean);
        }
        if self.mean > self.max {
            panic!("Mean {} greater than max {}", self.mean, self.max);
        }
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
            write!(f, "{}={}", key, value.lock().unwrap())?;
            if iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

pub fn parse_entry(value: &str, size: usize, station_list: Arc<RwLock<Stations>>) {
    let temp = value.rfind(';').expect("Failed to find ; in row");
    let entry = Entry {
        station: value[..temp].to_string(),
        temp: format!(
            "{}{}",
            value[temp + 1..size - 2].to_string(),
            value[size - 1..=size - 1].to_string()
        )
        .parse()
        .unwrap(),
    };

    // Acquire read lock to check if station exists
    let station_arc = {
        let read_lock = station_list.read().unwrap();
        read_lock.0.get(&entry.station).cloned() // Clone the Arc<Mutex<Station>> if it exists
    };

    if let Some(sl) = station_arc {
        // println!("Getting station lock: {}", entry.station);
        let mut s = sl.lock().unwrap();
        s.mean = (s.mean * s.count + entry.temp) / (s.count + 1);
        s.max = s.max.max(entry.temp);
        s.min = s.min.min(entry.temp);
        s.count += 1;
        return;
    }

    // println!("Adding station: {}", entry.station);
    let mut sl_w = station_list.write().expect("Error getting station list");
    // println!("Write lock acquired");
    sl_w.0.insert(
        entry.station,
        Arc::new(Mutex::new(Station {
            min: entry.temp,
            mean: entry.temp,
            max: entry.temp,
            count: 1,
        })),
    );
}
