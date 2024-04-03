use flate2::{write::DeflateEncoder, Compression};
use rand::{Rng, thread_rng};
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    // Example usage
    let input_file = "example.txt";
    let output_file = "compressed.bin";

    if let Err(err) = compress_file(input_file, output_file) {
        eprintln!("Error: {}", err);
    }
}

fn compress_file(input_file: &str, output_file: &str) -> io::Result<()> {
    // Read input file
    let mut input = File::open(input_file)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    // Generate random seed
    let mut rng = thread_rng();
    let seed: u64 = rng.gen();

    // Compress data using Deflate
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed_data = encoder.finish()?;

    // Write compressed data to output file
    let mut output = File::create(output_file)?;
    output.write_all(&compressed_data)?;

    println!("Compression successful with random seed: {}", seed);

    Ok(())
}
