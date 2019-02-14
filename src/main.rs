use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

// delay (in milliseconds between reading /proc/stat)
const DEFAULT_DELAY: u64 = 1000;

// read the first NUM_COLS_IN_STAT columns from /proc/stat
// after the cpuN column
const NUM_COLS_IN_STAT: usize = 7;

fn main() {
    let delay: u64 = if env::args().len() > 1 {
        match env::args().collect::<Vec<String>>()[1].parse::<u64>() {
            Ok(0..=99) => 100,
            Ok(d) => d,
            Err(_) => DEFAULT_DELAY,
        }
    } else {
        DEFAULT_DELAY
    };

    let t0 = read_values();
    thread::sleep(time::Duration::from_millis(delay));
    let t1 = read_values();

    let diff = diff_values(t0, t1);

    for cpu in diff.chunks(NUM_COLS_IN_STAT) {
        let load = cpu_load(cpu);
        // we have 8 unicode characters representing blocks, 0x2581 - 0x2588
        // so we partition '100% load' into 1 (0x2581) + 7 parts
        print!(
            "{}",
            std::char::from_u32(0x2581 + load / (100 / 7)).unwrap()
        );
    }
}

fn read_values() -> Vec<u32> {
    let mut f = File::open("/proc/stat").expect("no /proc/stat ?");
    let mut raw = String::new();
    f.read_to_string(&mut raw)
        .expect("error while reading /proc/stat");

    let mut all_cpus: Vec<u32> = Vec::new();
    for line in raw.lines().skip(1) {
        if line.starts_with("intr") {
            break;
        }
        let mut cols: Vec<&str> = line.split(' ').collect();
        all_cpus.append(
            &mut cols[1..=NUM_COLS_IN_STAT]
                .iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect(),
        );
    }
    all_cpus
}

fn diff_values(t0: Vec<u32>, t1: Vec<u32>) -> Vec<u32> {
    t0.iter().zip(t1.iter()).map(|(t0, t1)| t1 - t0).collect()
}

fn cpu_load(cols: &[u32]) -> u32 {
    let sum: u32 = cols.to_vec().iter().sum();
    let idle_share: u32 = 100 * cols[3] / sum;
    100 - idle_share
}
