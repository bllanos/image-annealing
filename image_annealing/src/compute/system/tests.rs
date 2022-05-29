use super::System;
use crate::ImageDimensions;

fn create_system_single_pixel() -> System {
    System::new(&ImageDimensions::new(1, 1).unwrap()).unwrap()
}

mod operation_count_swap {
    use std::error::Error;

    #[test]
    fn no_preceding_swaps() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.operation_count_swap(),
            "no swap passes have occurred since the last count swap operation",
        );
        Ok(())
    }
}

mod output_count_swap {
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.output_count_swap(),
            "no current output swap counts exist",
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
