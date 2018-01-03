#![feature(exclusive_range_pattern)]
extern crate time;
extern crate rand;

use rand::Rng;
use std::time::Instant;

fn generate(rng: &mut FnMut(&mut [u8]), alphabet: &str, size: usize) -> Result<String, &'static str> {
    match (alphabet.len(), size) {
        (1..256, s) if (s >= 1)  => {
            let mask: u8 = (2isize << ( ((alphabet.len() - 1) as f64).log(10f64) / 2f64.log(10f64) ).floor() as usize) as u8 - 1u8;
            let step = (1.6f64 * (mask as f64) * (size as f64) / (alphabet.len()) as f64).ceil() as usize;
            let mut result_string = String::from("");
            loop {
                let mut bytes: Vec<u8> = vec![0u8; step];
                rng( bytes.as_mut_slice() );
                for i in 0..step {
                    let alphabet_index = (bytes[i] & mask) as usize;
                    if alphabet_index >= alphabet.len() { continue; };
                    let c = alphabet.as_bytes()[alphabet_index] as char;
                    result_string.push( c );
                    if result_string.len() == size { return Ok( result_string ); };
                }
            }
        },
        (_, 0) => return Err("Size must be greater than 0."),
        (_, _) => return Err("Alphabet length must be within 1 and 255 characters."),
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut ff = |bytes: &mut [u8]| rng.fill_bytes(bytes);
    let alphabet = "_~0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    // Bench
    {
        let mut stats: Vec<f64> = Vec::new();
        const ITERATIONS: usize = 1_000_000;
        for _ in 0..ITERATIONS  {
            let now = Instant::now();
            {
                let _ignore = generate(&mut ff, &alphabet, 21);
            }
            let elapsed = now.elapsed();
            let elapsed_ms = (elapsed.as_secs() as f64 * 1_000.00f64) + (elapsed.subsec_nanos() as f64 / 1_000_000.00f64);
            &stats.push(elapsed_ms.clone());
        }
        let total_time_msec: f64 = stats.iter().sum();
        let total_time_sec: f64 = total_time_msec / 1_000f64;
        let average_time_sec: f64 = total_time_sec / ITERATIONS as f64;
        let ops_per_sec: f64 = 1.0f64 / average_time_sec;
        
        println!("Performance test");
        let s = generate(&mut ff, &alphabet, 21);
        match s {
            Ok(id) => println!("Nanoid generation test: {}", id),
            Err(e) => println!("Nanoid generation test: Cannot generate nanoid: {}", e),
        };
        println!("Total time of {} nanoid generation tests:\t{:.4} sec", ITERATIONS, total_time_sec);
        println!("Nanoids generated in 1 sec, average:\t{:.2} ops/sec", ops_per_sec);
        println!("", );
    }
}
