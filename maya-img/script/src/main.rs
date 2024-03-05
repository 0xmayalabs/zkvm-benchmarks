use image::{GenericImageView};
use sp1_core::{SP1Verifier, utils};
use sp1_core::{SP1Prover, SP1Stdin};
use std::time::Instant;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // Original Image
    let img_original = image::open("./src/original.png").unwrap();
    let (width, height) = img_original.dimensions();
    println!("Original dimensions {} {}", width, height);
    let original_img_buffer = img_original.into_bytes();

    // Final Image
    let img_final = image::open("./src/cropped.png").unwrap();
    let (width, height) = img_final.dimensions();
    println!("Final dimensions {} {}", width, height);
    let final_img_buffer = img_final.into_bytes();

    // Write data.
    stdin.write(&original_img_buffer);
    stdin.write(&final_img_buffer);

    let start = Instant::now();

    println!("Starting proof generation!!");
    let proof = SP1Prover::prove(ELF, stdin).expect("Proving failed");
    println!("Proof generation done!!");

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("Proof verification done!!");

    proof.save("proof.json").expect("Save proof.json");

    // Record the end time
    let end = Instant::now();

    // Calculate and print the elapsed time
    let duration = end.duration_since(start).as_secs();
    println!("Time elapsed {} seconds", duration);
}