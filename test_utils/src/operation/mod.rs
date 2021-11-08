use image_annealing::compute::conversion::VectorFieldEntry;
use image_annealing::compute::format::VectorFieldImageBuffer;

pub fn swap(vector_field: &VectorFieldImageBuffer) -> VectorFieldImageBuffer {
    let width = vector_field.width();
    let width_less_one = width
        .checked_sub(1u32)
        .expect("Vector field width is zero.");
    VectorFieldImageBuffer::from_fn(width, vector_field.height(), |x, y| {
        if x % 2 == 0 {
            if x < width_less_one {
                let input_pixel = vector_field.get_pixel(x + 1, y);
                let mut entry = VectorFieldEntry::from_pixel(input_pixel);
                entry.0 += 1i16;
                entry.to_pixel()
            } else {
                *vector_field.get_pixel(x, y)
            }
        } else {
            let input_pixel = vector_field.get_pixel(x - 1, y);
            let mut entry = VectorFieldEntry::from_pixel(input_pixel);
            entry.0 -= 1i16;
            entry.to_pixel()
        }
    })
}
