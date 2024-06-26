use image::{GenericImageView, ImageFormat};
use image::imageops::{crop_imm, resize, FilterType};
use std::cmp::min;

pub fn crop_and_resize(path: String) {
	let img = image::open(path.clone()).unwrap();

	let width = img.dimensions().0;
	let height = img.dimensions().1;
	let temp_size = min(width, height);
	
	let mid_x = width/2;
	let mid_y = height/2;

	let final_x = mid_x - temp_size/2;
	// if final_x == 0 {
	// 	final_x += 0; wtf
	// }

	let final_y = mid_y - temp_size/2;
	// if final_y == 0 {
	// 	final_y += 0; przez chwile bylo potrzebne a teraz juz chyba nie?
	// }

	// println!("{} {} {}", mid_x, mid_y, temp_size);
	let cropped_img = crop_imm(&img, final_x, final_y, temp_size, temp_size);
	// println!("{} {} {} {}", final_x,  final_y, temp_size, temp_size);
	cropped_img
            .to_image()
            .save_with_format(&path, ImageFormat::Png)
            .unwrap();

    let img = image::open(path.clone()).unwrap();
    let resized_img = resize(&img, 128, 128, FilterType::Lanczos3);

    resized_img.save_with_format(&path, ImageFormat::Png).unwrap();

}