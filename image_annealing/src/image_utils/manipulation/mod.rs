use super::validation::ValidatedPermutation;
use crate::compute::conversion::VectorFieldEntry;
use crate::compute::format::VectorFieldImageBuffer;

pub(crate) fn invert_permutation(permutation: &ValidatedPermutation) -> VectorFieldImageBuffer {
    let vector_field: &VectorFieldImageBuffer = permutation.as_ref();
    let mut inverse = VectorFieldImageBuffer::new(vector_field.width(), vector_field.height());
    for (x, y, px) in vector_field.enumerate_pixels() {
        let delta = VectorFieldEntry::from_pixel(px);
        let target_coordinates: (u32, u32) = (
            (<i32 as TryFrom<u32>>::try_from(x).unwrap() + i32::from(delta.0))
                .try_into()
                .unwrap(),
            (<i32 as TryFrom<u32>>::try_from(y).unwrap() + i32::from(delta.1))
                .try_into()
                .unwrap(),
        );
        let inverse_entry = VectorFieldEntry(-delta.0, -delta.1);
        inverse.put_pixel(
            target_coordinates.0,
            target_coordinates.1,
            inverse_entry.to_pixel(),
        );
    }
    inverse
}

#[cfg(test)]
mod tests;
