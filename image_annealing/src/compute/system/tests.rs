use super::System;
use crate::ImageDimensions;

fn create_system_single_pixel() -> System {
    futures::executor::block_on(System::new(&ImageDimensions::new(1, 1).unwrap())).unwrap()
}

mod operation_count_swap {
    use super::super::super::link::swap::{SwapPass, SwapPassSequence};
    use super::super::{DevicePollType, SwapOperationInput};
    use crate::DisplacementGoal;
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
        test_utils::assert_error_contains(
            system.operation_count_swap(pass.into()),
            "not all selected swap passes have occurred since the last count swap operation",
        );
        Ok(())
    }
}

mod output_count_swap {
    use super::super::super::link::swap::SwapPass;
    use super::super::{DevicePollType, SwapOperationInput};
    use crate::DisplacementGoal;
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
        test_utils::assert_error_contains(
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
        test_utils::assert_error_contains(
            futures::executor::block_on(system.output_count_swap(DevicePollType::Wait,&SwapPass::Vertical.into())),
            "not all selected swap passes were counted during the last count swap operation, if one was performed",
        );
        Ok(())
    }
}

mod output_permutation {
    use super::super::DevicePollType;
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            futures::executor::block_on(system.output_permutation(DevicePollType::Wait)),
            "an output permutation does not exist or has been invalidated",
        );
        Ok(())
    }
}

mod output_permuted_image {
    use super::super::super::output::format::ImageFormat;
    use super::super::DevicePollType;
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            futures::executor::block_on(
                system.output_permuted_image(DevicePollType::Wait, ImageFormat::Rgba8),
            ),
            "an output image does not exist or has been invalidated",
        );
        Ok(())
    }
}
