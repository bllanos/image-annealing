use super::System;
use crate::ImageDimensions;

fn create_system_single_pixel() -> System {
    System::new(&ImageDimensions::new(1, 1).unwrap()).unwrap()
}

mod operation_count_swap {
    use super::super::super::link::swap::SwapPassSequence;
    use std::error::Error;

    #[test]
    fn no_preceding_swaps() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.operation_count_swap(SwapPassSequence::all()),
            "not all selected swap passes have occurred since the last count swap operation",
        );
        Ok(())
    }
}

mod output_count_swap {
    use super::super::super::link::swap::SwapPass;
    use super::super::SwapOperationInput;
    use crate::DisplacementGoal;
    use std::error::Error;

    #[test]
    fn no_preceding_count_swap_operation() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        let pass = SwapPass::Horizontal;
        system.operation_create_permutation()?;
        let permutation = system.output_permutation()?;
        system.operation_swap(&SwapOperationInput {
            pass,
            acceptance_threshold: Default::default(),
            permutation: None,
            displacement_goal: Some(&DisplacementGoal::from_vector_field(
                permutation.into_inner(),
            )?),
        })?;
        test_utils::assert_error_contains(
            system.output_count_swap(&pass.into()),
            "not all selected swap passes were counted during the last count swap operation, if one was performed",
        );
        Ok(())
    }
}

mod output_permutation {
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.output_permutation(),
            "an output permutation does not exist or has been invalidated",
        );
        Ok(())
    }
}

mod output_permuted_image {
    use super::super::super::output::format::ImageFormat;
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.output_permuted_image(ImageFormat::Rgba8),
            "an output image does not exist or has been invalidated",
        );
        Ok(())
    }
}
