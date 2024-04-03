use flate2::{write::DeflateEncoder, Compression};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    // Example usage
    let input_file = "example.txt";
    let output_file = "compressed.bin";
    let seed = 123456789; // Seed for the pseudorandom number generator

    if let Err(err) = compress_file(input_file, output_file, seed) {
        eprintln!("Error: {}", err);
    }
}

fn compress_file(input_file: &str, output_file: &str, seed: u64) -> io::Result<()> {
    // Read input file
    let mut input = File::open(input_file)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    // Initialize PRNG with seed
    let mut rng = SmallRng::seed_from_u64(seed);

    // Shuffle or modify data using PRNG (optional)

    // Compress data using Deflate
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed_data = encoder.finish()?;

    // Write compressed data to output file
    let mut output = File::create(output_file)?;
    output.write_all(&compressed_data)?;

    Ok(())
}
