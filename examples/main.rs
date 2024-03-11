#![allow(missing_docs)]

use contest::find_submatrices;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use palette::IntoColor;

fn main() {
    let (grid, _k) = test_utils::generate_test_grid();

    let mut submatrices = find_submatrices(&grid, i32::MAX / 2.0 as i32);

    println!("number of submatrices: {}", submatrices.len());

    let max_x = submatrices.iter().max_by_key(|&a| a.1).unwrap().1;
    let max_y = submatrices.iter().max_by_key(|&a| a.2).unwrap().2;
    let max_sum = submatrices.iter().max_by_key(|&a| a.0).unwrap().0 as u64;

    let mut img = RgbImage::new((max_x) as u32, (max_y) as u32);

    submatrices.sort_by_key(|a| a.0); // sort by submatrix sum

    for submatrix in submatrices.into_iter().rev() {
        let (sum, x, y) = submatrix;
        let rgb: palette::Srgb =
            palette::Hsv::new((sum as f32 / max_sum as f32) * 360.0, 1.0, 1.0).into_color();
        let (red, green, blue) = rgb.into_format::<u8>().into_components();
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(x as u32 + 1, y as u32 + 1),
            Rgb([red, green, blue]),
        );
    }

    img.save("test.png").unwrap();
}
