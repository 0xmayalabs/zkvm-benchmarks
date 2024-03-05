use image::{GenericImageView};
use sp1_core::{SP1Verifier, utils};
use sp1_core::{SP1Prover, SP1Stdin};
use std::time::Instant;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    // crop_zk_circuit();
    crop_transformation();
}

// crop_transformation crops an image and proves the computation of cropping.
fn crop_transformation() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // Original Image
    let img_original = image::open("./src/original.png").unwrap();
    let (width, height) = img_original.dimensions();
    println!("Original dimensions {} {}", width, height);
    let original_img_buffer = img_original.into_bytes();
    let new_width: u32 = 5;
    let new_height: u32 = 5;

    stdin.write(&original_img_buffer);
    stdin.write(&width);
    stdin.write(&height);
    stdin.write(&new_width);
    stdin.write(&new_height);

    let start = Instant::now();

    println!("Starting proof generation!!");
    let mut proof = SP1Prover::prove(ELF, stdin).expect("Proving failed");
    println!("Proof generation done!!");

    // Record the end time
    let end = Instant::now();

    // Calculate and print the elapsed time
    let duration = end.duration_since(start).as_secs();
    println!("Time elapsed {} seconds", duration);

    // Read transformed image.
    let transformed_img_buf = proof.stdout.read::<Vec<u8>>();
    let new_width = proof.stdout.read::<u32>();
    let new_height = proof.stdout.read::<u32>();

    println!(
        "transformed_img_bug.len() = {:?}",
        transformed_img_buf.len()
    );
    println!("new_width = {:?}", new_width);
    println!("new_height = {:?}", new_height);
}

// crop_zk_circuit implements the ZK circuit to prove crop transformation.
fn crop_zk_circuit() {
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

    // Record the end time
    let end = Instant::now();

    // Calculate and print the elapsed time
    let duration = end.duration_since(start).as_secs();
    println!("Time elapsed {} seconds", duration);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");
    println!("Proof verification done!!");

    proof.save("proof.json").expect("Save proof.json");
}