#![allow(non_snake_case)]

static BINARYARRAY : [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];

struct SDR {
    composition: [MicroSDR],
    data: [u8]
}

enum MicroSDR {
    Quantitative {n: usize, w: usize, mix: usize, max: usize, periodic: bool},
    QuantitativeRDSE,
    Qualitative {x: u8},
}

fn main() {
    let mut x : [u8; 4] = [0, 1, 2, 31];

    flipNthBit(1, &mut x);

    printSDR(&x);
}

fn flipNthBit(index : usize, sdr : &mut [u8]) {

    let sdrIdx = index / 8;
    let subIdx = index % 8;

    sdr[sdrIdx] = sdr[sdrIdx] ^ BINARYARRAY[subIdx];
}

fn printSDR(sdr : &[u8]) {

    println!("----------");

    for i in 0..4
    {
        let j = sdr[i];
        println!("|{}|", format!("{:0>8}", format!("{j:b}") ) );
    }

    println!("----------");
}