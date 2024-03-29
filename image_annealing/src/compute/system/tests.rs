use super::System;
use crate::ImageDimensions;

fn create_system_single_pixel() -> System {
    futures::executor::block_on(System::new(&ImageDimensions::try_new(1, 1).unwrap())).unwrap()
}

mod operation_count_swap {
    use super::super::super::link::swap::{SwapPass, SwapPassSequence};
    use super::super::{DevicePollType, SwapOperationInput};
    use crate::{DisplacementGoal, VectorField};
    use std::error::Error;

    #[test]
    fn no_preceding_swaps() {
        let mut system = super::create_system_single_pixel();
        test_util::assert_error_contains(
            system.operation_count_swap(SwapPassSequence::all()),
            "not all selected swap passes have occurred since the last count swap operation",
        );
    }

    #[test]
    fn missing_preceding_swaps_since_last_count() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        let pass = SwapPass::Horizontal;
        system.operation_create_permutation()?;
        let permutation =
            futures::executor::block_on(system.output_permutation(DevicePollType::Wait))?;
        system.operation_swap(&SwapOperationInput {
            pass,
            acceptance_threshold: Default::default(),
            permutation: None,
            displacement_goal: Some(&DisplacementGoal::from_vector_field(
                permutation.into_inner(),
            )?),
        })?;
        system.operation_count_swap(pass.into())?;
        system.operation_swap(&SwapOperationInput {
            pass: SwapPass::Vertical,
            acceptance_threshold: Default::default(),
            permutation: None,
            displacement_goal: None,
        })?;
        test_util::assert_error_contains(
            system.operation_count_swap(pass.into()),
            "not all selected swap passes have occurred since the last count swap operation",
        );
        Ok(())
    }
}

mod output_count_swap {
    use super::super::super::link::swap::SwapPass;
    use super::super::{DevicePollType, SwapOperationInput};
    use crate::{DisplacementGoal, VectorField};
    use std::error::Error;

    #[test]
    fn no_preceding_count_swap_operation() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        let pass = SwapPass::Horizontal;
        system.operation_create_permutation()?;
        let permutation =
            futures::executor::block_on(system.output_permutation(DevicePollType::Wait))?;
        system.operation_swap(&SwapOperationInput {
            pass,
            acceptance_threshold: Default::default(),
            permutation: None,
            displacement_goal: Some(&DisplacementGoal::from_vector_field(
                permutation.into_inner(),
            )?),
        })?;
        test_util::assert_error_contains(
            futures::executor::block_on(system.output_count_swap(DevicePollType::Wait, &pass.into())),
            "not all selected swap passes were counted during the last count swap operation, if one was performed",
        );
        Ok(())
    }

    #[test]
    fn missing_swaps_in_last_count() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        let pass = SwapPass::Horizontal;
        system.operation_create_permutation()?;
        let permutation =
            futures::executor::block_on(system.output_permutation(DevicePollType::Wait))?;
        system.operation_swap(&SwapOperationInput {
            pass,
            acceptance_threshold: Default::default(),
            permutation: None,
            displacement_goal: Some(&DisplacementGoal::from_vector_field(
                permutation.into_inner(),
            )?),
        })?;
        system.operation_count_swap(pass.into())?;
        test_util::assert_error_contains(
            futures::executor::block_on(system.output_count_swap(DevicePollType::Wait,&SwapPass::Vertical.into())),
            "not all selected swap passes were counted during the last count swap operation, if one was performed",
        );
        Ok(())
    }
}

mod output_displacement_goal {
    use super::super::DevicePollType;

    #[test]
    fn no_preceding_operations() {
        let mut system = super::create_system_single_pixel();
        test_util::assert_error_contains(
            futures::executor::block_on(system.output_displacement_goal(DevicePollType::Wait)),
            "an output displacement goal field does not exist or has been invalidated",
        );
    }
}

mod output_permutation {
    use super::super::super::format::{LosslessImage, Rgba16Image};
    use super::super::{DevicePollType, PermuteOperationInput, System};
    use crate::image_utils::validation;
    use crate::{ImageDimensions, ImageDimensionsHolder, VectorField};
    use std::error::Error;
    use test_util::permutation::DimensionsAndPermutation;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        let output = futures::executor::block_on(system.output_permutation(DevicePollType::Wait))?;
        assert_eq!(output.dimensions(), &ImageDimensions::try_new(1, 1)?);
        assert!(output.is_identity());
        Ok(())
    }

    #[test]
    fn after_input_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::non_identity();
        let image = LosslessImage::Rgba16(Rgba16Image::new(
            test_util::image::coordinates_to_colors(&dimensions),
        )?);
        let mut system = futures::executor::block_on(System::new(&ImageDimensions::try_new(
            dimensions.width(),
            dimensions.height(),
        )?))?;
        system.operation_permute(&PermuteOperationInput {
            image: Some(&image),
            permutation: Some(&unsafe {
                validation::vector_field_into_validated_permutation_unchecked(permutation)
            }),
        })?;
        test_util::assert_error_contains(
            futures::executor::block_on(system.output_permutation(DevicePollType::Wait)),
            "an output permutation does not exist or has been invalidated",
        );
        Ok(())
    }
}

mod output_permuted_image {
    use super::super::super::output::format::ImageFormat;
    use super::super::DevicePollType;

    #[test]
    fn no_preceding_operations() {
        let mut system = super::create_system_single_pixel();
        test_util::assert_error_contains(
            futures::executor::block_on(
                system.output_permuted_image(DevicePollType::Wait, ImageFormat::Rgba8),
            ),
            "an output image does not exist or has been invalidated",
        );
    }
}
