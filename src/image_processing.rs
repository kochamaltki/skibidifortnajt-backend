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

	let cropped_img = crop_imm(&img, mid_x-temp_size/2, mid_y-temp_size/2, mid_x+temp_size/2, mid_y+temp_size/2);
	cropped_img
            .to_image()
            .save_with_format(&path, ImageFormat::Png)
            .unwrap();

    let img = image::open(path.clone()).unwrap();
    let resized_img = resize(&img, 128, 128, FilterType::Lanczos3);

    resized_img.save_with_format(&path, ImageFormat::Png).unwrap();

}