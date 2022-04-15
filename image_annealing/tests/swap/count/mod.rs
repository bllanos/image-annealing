mod sum_at_least_two_elements_per_invocation {
    use super::super::border;
    use image_annealing_shaders::WorkgroupDimensions;
    use std::convert::TryInto;
    use std::error::Error;

    fn at_least_two_elements_per_invocation() -> usize {
        let workgroup_dimensions = WorkgroupDimensions::count_swap();
        (workgroup_dimensions
            .invocation_count()
            .checked_mul(3)
            .unwrap()
            .checked_sub(1)
            .unwrap())
        .try_into()
        .unwrap()
    }

    #[test]
    fn long_accept_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(at_least_two_elements_per_invocation(), 3, true)
    }

    #[test]
    fn long_reject_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(at_least_two_elements_per_invocation(), 3, false)
    }

    #[test]
    fn tall_accept_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(3, at_least_two_elements_per_invocation(), true)
    }

    #[test]
    fn tall_reject_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(3, at_least_two_elements_per_invocation(), false)
    }
}
