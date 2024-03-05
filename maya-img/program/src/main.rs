use image::{GenericImageView, ImageBuffer, imageops, Pixel, RgbImage};

pub fn main() {
    // crop_zk_circuit();
    crop_transformation();
}

// crop_transformation crops an image and proves the computation of cropping.
fn crop_transformation() {
    let img_buf = sp1_zkvm::io::read::<Vec<u8>>();
    let old_width = sp1_zkvm::io::read::<u32>();
    let old_height = sp1_zkvm::io::read::<u32>();
    let new_width = sp1_zkvm::io::read::<u32>();
    let new_height = sp1_zkvm::io::read::<u32>();

    // crop image.
    println!("Before reading image");
    println!("width: {:?}", old_width);
    println!("height: {:?}", old_height);
    println!("img_buf.len(): {:?}", img_buf.len());
    let img: RgbImage = ImageBuffer::from_raw(old_width, old_height, img_buf).unwrap();
    println!("Finished reading image");

    let mut transformed_img = img;
    transformed_img = imageops::crop(
        &mut transformed_img,
        0,
        0,
        new_width,
        new_height,
    ).to_image();

    let img_buffer = transformed_img.as_raw();

    // Write back cropped image.
    sp1_zkvm::io::write(&img_buffer);
    sp1_zkvm::io::write(&new_width);
    sp1_zkvm::io::write(&new_height);
}

// crop_zk_circuit implements the ZK circuit to prove crop transformation.
fn crop_zk_circuit() {
    let original_img_buf = sp1_zkvm::io::read::<Vec<u8>>();
    let final_img_buf = sp1_zkvm::io::read::<Vec<u8>>();

    // Original image
    let original_img = image::load_from_memory(&original_img_buf).expect("load original image");

    let (original_width, original_height) = original_img.dimensions();
    println!("Original dimensions {} {}", original_width, original_height);

    // Initialize a 3D vector to hold the pixel values
    let mut original_pixel_values: Vec<Vec<Vec<u8>>> = Vec::with_capacity(original_height as usize);

    for y in 0..original_height {
        let mut row: Vec<Vec<u8>> = Vec::with_capacity(original_width as usize);
        for x in 0..original_width {
            let pixel = original_img.get_pixel(x, y);
            let channels = pixel.channels();

            // Assuming the image is RGB or RGBA, channels will contain 3 (RGB) or 4 (RGBA) values
            row.push(channels.to_vec());
        }
        original_pixel_values.push(row);
    }

    // Final image
    let final_img = image::load_from_memory(&final_img_buf).expect("load final image");

    let (final_width, final_height) = final_img.dimensions();
    println!("Final dimensions {} {}", final_width, final_height);

    // Initialize a 3D vector to hold the pixel values
    let mut final_pixel_values: Vec<Vec<Vec<u8>>> = Vec::with_capacity(final_height as usize);

    for y in 0..final_height {
        let mut row: Vec<Vec<u8>> = Vec::with_capacity(final_width as usize);
        for x in 0..final_width {
            let pixel = final_img.get_pixel(x, y);
            let channels = pixel.channels();

            // Assuming the image is RGB or RGBA, channels will contain 3 (RGB) or 4 (RGBA) values
            row.push(channels.to_vec());
        }
        final_pixel_values.push(row);
    }

    let ok: bool = is_cropped(&original_pixel_values, &final_pixel_values, 0, 0);
    if ok {
        println!("All ok in zkVM!!");
    } else {
        println!("Fatal error!!");
    }
}

// is_cropped returns true if arr2 is a cropped version of arr1.
fn is_cropped(arr1: &Vec<Vec<Vec<u8>>>, arr2: &Vec<Vec<Vec<u8>>>, x: usize, y: usize) -> bool {
    let height2 = arr2.len();
    let width2 = arr2[0].len();

    // Check if arr2 can fit inside arr1 starting from (x, y)
    if y + height2 > arr1.len() || x + width2 > arr1[0].len() {
        return false;  // arr2 can't fit, so it's not a cropped version
    }

    for (i, row) in arr2.iter().enumerate() {
        for (j, pixel) in row.iter().enumerate() {
            // Compare the pixel values in arr2 with the corresponding pixels in arr1
            if arr1[y + i][x + j] != *pixel {
                return false;  // Found a mismatch, so arr2 is not a cropped version of arr1
            }
        }
    }

    true  // All pixels matched, arr2 is a cropped version of arr1
}
