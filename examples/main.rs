//! This program visualises the sums of all submatrices in a grid filled with
//! random numbers.

use contest::find_submatrices;
use image::{imageops::rotate90, Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use palette::{FromColor, Hsv, Srgb};

/// This programs visualises the sums of all submatrices in a grid filled with
/// random numbers. It is "order in the chaos" in a way.
fn main() {
    let (grid, _k) = test_utils::generate_test_grid();

    // Let's make sure that all submatrices in the grid are valid submatrices
    // by using the highest possible k value.
    let mut submatrices = find_submatrices(&grid, i32::MAX);

    println!("number of submatrices: {}", submatrices.len());

    let max_x = submatrices.iter().max_by_key(|(_, x, _)| *x).unwrap().1;
    let max_y = submatrices.iter().max_by_key(|(_, _, y)| *y).unwrap().2;
    let max_sum = submatrices.iter().max_by_key(|(sum, _, _)| *sum).unwrap().0 as u64;

    let mut img = RgbImage::new((max_x) as u32, (max_y) as u32);

    submatrices.sort_by_key(|(sum, _, _)| *sum ); // Sort by submatrix sum
                                                        // so that the smallest 
                                                        // submatrices are drawn last.

    for (sum, x, y) in submatrices.into_iter().rev() {
        // Map the sum to a color on the rainbow.
        // Note that as hsv is a circle over all colors, the lowest value is red, 
        // and the highest is red again. But we can't see ulraviolet anyway (and
        // luckily monitors won't display it either).
        let (red, green, blue) =
            Srgb::from_color(Hsv::new((sum as f32 / max_sum as f32) * 360.0, 1.0, 1.0))
                .into_format::<u8>()
                .into_components();
        // Draw the submatrix using the color representing the sum.
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, 0).of_size(x as u32 + 1, y as u32 + 1),
            Rgb([red, green, blue]),
        );
    }

    // produce three further quadrants for symmetry
    let q2 = rotate90(&img);
    let q3 = rotate90(&q2);
    let q4 = rotate90(&q3);
    let (width, height) = img.dimensions();

    let mut imgbuf: RgbImage = RgbImage::new((max_y as u32) * 2, (max_x as u32) * 2);
    image::imageops::replace(&mut imgbuf, &img, 0, 0);
    image::imageops::replace(&mut imgbuf, &q2, width as i64, 0);
    image::imageops::replace(&mut imgbuf, &q3, width as i64, height as i64);
    image::imageops::replace(&mut imgbuf, &q4, 0, height as i64);

    // and create it as an png file
    imgbuf.save("test.png").unwrap();
}
