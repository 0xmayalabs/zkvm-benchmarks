use image::{GenericImageView, Pixel};

pub fn main() {
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
