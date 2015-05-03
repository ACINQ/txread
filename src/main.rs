extern crate rustc_serialize;

use std::env;
use rustc_serialize::hex::{FromHex, ToHex};

fn main() {
    if let Some(tx_hex) = env::args().nth(1) {
        let tx_bytes = tx_hex.from_hex().unwrap();
        parse(tx_bytes);
    } else {
            println!("usage:  txread <serialized_tx>");
    }
}

fn parse(tx: Vec<u8>) {
    let mut i: usize = 0;
    println!("version: {:?}", &tx[i..i+4].to_hex()); i+=4;
    println!("");
    assert!(tx[i] < 0xFD, "cannot parse tx with more than 253 inputs");
    let nb_inputs = tx[i]; i+=1;
    for j in 0..nb_inputs {
        println!("input #{:?}", j);
        println!("    hash: {:?}", &tx[i..i+32].to_hex()); i+=32;
        println!("    index: {:?}", &tx[i..i+4].to_hex()); i+=4;
        assert!(tx[i] <= 0xFD, "cannot parse tx scripts encoded on more than 2 bytes");
        let script_size: usize =   if tx[i] < 0xFD {
                                let s = tx[i] as usize; i+=1; s
                            } else {
                                let s = tx[i] as usize * 256 + tx[i+1] as usize; i+=2; s
                            };
        println!("    script size: {:?} B", script_size);
        println!("    script: {:?}", &tx[i..i+script_size].to_hex()); i+=script_size;
        println!("    sequence: {:?}", &tx[i..i+4].to_hex()); i+=4;
    }
    println!("");
    assert!(tx[i] < 0xFD, "cannot parse tx with more than 253 outputs");
    let nb_outputs = tx[i]; i+=1;
    for j in 0..nb_outputs {
        println!("output #{:?}", j);
        println!("    amount: {:?}", &tx[i..i+8].to_hex()); i+=8;
        assert!(tx[i] <= 0xFD, "cannot parse tx scripts encoded on more than 2 bytes");
        let script_size: usize =   if tx[i] < 0xFD {
                                let s = tx[i] as usize; i+=1; s
                            } else {
                                let s = tx[i] as usize * 256 + tx[i+1] as usize; i+=2; s
                            };
        println!("    script size: {:?} B", script_size);
        println!("    script: {:?}", &tx[i..i+script_size].to_hex()); i+=script_size;
    }
    println!("");
    println!("locktime: {:?}", &tx[i..i+4].to_hex()); i+=4;
    assert!(i == tx.len(), "unread data at the end of the input");
}

#[test]
fn read_test() {
    parse("0100000001eac8c405a5747c91f438ff1a2e9ef3de6280541f8ae50dee8af744bbdb75064f010000006a4730440220627558dd3705962d2414c48cb30c0f0c5343be551ae0ce7ba0a69c722f09168602200e0ae4158b9d98049ed706d671ff8ee7e881ecbf3e65e32f320abe68985458e00121039b8ac6ae0e5ccf32c5c996259cf98328d51a941746c32057d26ca33cb8a3df36ffffffff02f865a7e0000000001976a9144635a711ad845e08f439e23db2843d03528a7c8188ac00c2eb0b000000001976a9142ce3b176d2cb46ce93299b0e308990e630d160c688ac00000000".from_hex().unwrap());
}