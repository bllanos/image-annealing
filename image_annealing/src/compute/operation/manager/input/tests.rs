mod swap_operation_input {
    use super::super::super::super::super::link::swap::SwapPass;
    use super::super::SwapOperationInput;

    #[test]
    fn from_pass_and_threshold() {
        let pass = SwapPass::OffsetVertical;
        let acceptance_threshold = 2.0;
        let input = SwapOperationInput::from_pass_and_threshold(
            SwapPass::OffsetVertical,
            acceptance_threshold,
        );
        assert_eq!(
            input,
            SwapOperationInput {
                pass,
                acceptance_threshold,
                permutation: None,
                displacement_goal: None,
            }
        );
    }
}
