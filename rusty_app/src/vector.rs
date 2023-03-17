#![allow(non_snake_case)]

use std::env;

static PRINT_WIDTH: u16 = 25;

static LENGTH: u16 = 4;
static MIN_VALUE: f64 = 1.0;
static MAX_VALUE: f64 = 5.0;
static POPULATION: u16 = 3;
static CYCLIC: bool = true;

static DATA_ENTRIES: u16;

struct Ruleset {
    length: u16,
    min_value: f64,
    max_value: f64,
    population: u16,
    cyclic: bool,

    sdrs: Vec<u16>,
}

impl Ruleset {

    // static
    fn construct() -> Ruleset {
        
    }
}

struct SDR {
    location: u16
}

impl SDR {

    fn printSDR(&self) {
        if (CYCLIC) { self.printCYCLIC() } else { self.printNONCYCLIC() }
    }

    fn printNONCYCLIC(&self) {
        for a in 0..LENGTH {

            if (a >= self.location) && (a < self.location + POPULATION) {
                print!("■");
            } else {
                print!("▨")
            }

            if ((a + 1) % PRINT_WIDTH) == 0 {
                println!("");
            } else {
                print!(" ");
            }

        }
    }

    fn printCYCLIC(&self) {

        if (self.location + POPULATION <= LENGTH) {
            self.printNONCYCLIC()
        } else {
            for a in 0..LENGTH {

                if (a < (self.location + POPULATION - LENGTH)) || ((a >= self.location) && (a < self.location + POPULATION)) {
                    print!("■");
                } else {
                    print!("▨")
                }

                if ((a + 1) % PRINT_WIDTH) == 0 {
                    println!("");
                } else {
                    print!(" ");
                }

            }
        }
    }

    fn calculateLocation(value: f64) -> u16 {

        let buckets: f64 = if (CYCLIC) {LENGTH} else {(LENGTH - POPULATION + 1)} as f64; // == # buckets
        let new_max: f64 = (MAX_VALUE - MIN_VALUE) as f64;

        let bucket: u16 = ((value - MIN_VALUE as f64) * (buckets / new_max)) as u16;

        // SPECIAL CASE: Since the max is inclusive, let that save to the last possible bucket
        return if (value == MAX_VALUE) {bucket - 1} else {bucket};
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let value: f64 = args[1].parse::<f64>().unwrap();

    println!("Input value is: {}, constrained by minimum {}, and maximum {}.", value, MIN_VALUE, MAX_VALUE);

    let sdr = SDR {
        location: SDR::calculateLocation(value),
        identifier: 0
    };

    sdr.printSDR();
}


/*
fn main() {

    let mut vector = Vec::new();
    vector.push(3);
    vector.push(4);

    changer(&mut vector[..]);

    println!("Vector: {}", vector[0]);
    
}

fn changer(sl : &mut [u8]) {
    sl[0] = 1;
}
*/